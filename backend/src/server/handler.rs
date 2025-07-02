use std::{
    pin::pin, str::FromStr, time::{Duration, Instant}
};

use actix_ws::{AggregatedMessage, CloseReason, CloseCode};
use futures_util::{
    StreamExt as _,
    future::{Either, select},
};
use serde_json::{from_str, Value};
use tokio::{sync::mpsc::{self, UnboundedReceiver}, time::interval};

use crate::server::game::player::PlayerId;

use super::{dto::{actions::UserAction, responses::ServerResponse}, game, server::{ConnId, GameServerHandle}};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn game_ws(
    game_server: GameServerHandle,
    mut session: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
    player_id: i32,
) {
    log::info!("New session connected");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    // connect: if connection with this player_id exists, replace it
    let conn_id = game_server.connect(player_id, conn_tx).await;

    let msg_stream = msg_stream
        .max_frame_size(128 * 1024)     // ~1Mb
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);    // ~16Mb

    let mut msg_stream = pin!(msg_stream);

    let player_id: PlayerId = player_id;

    // Send session info
    ServerResponse::SessionInfo { id: player_id, players: game_server.get_session_info().await }
        .send(&mut session).await.unwrap();

    // begin loop

    let close_reason = loop {
        // most of the futures we process need to be stack-pinned to work with select()

        let tick = pin!(interval.tick());
        let msg_rx = pin!(conn_rx.recv());

        // TODO: nested select is pretty gross for readability on the match
        let messages = pin!(select(msg_stream.next(), msg_rx));

        match select(messages, tick).await {

            // commands & messages received from client
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    AggregatedMessage::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        // unwrap:
                        session.pong(&bytes).await.unwrap();
                    }

                    AggregatedMessage::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    AggregatedMessage::Text(text) => {
                        let close_reason = process_received_text(&game_server, &mut session, &text, conn_id, player_id).await;
                        // break if process_received_text returned a close reason
                        if close_reason.is_some() {
                            break close_reason;
                        }
                    }

                    AggregatedMessage::Binary(_bin) => {
                        log::warn!("unexpected binary message");
                    }

                    AggregatedMessage::Close(reason) => break reason,
                }
            }

            // client WebSocket stream error
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                log::error!("{}", err);
                break None;
            }

            // client WebSocket stream ended
            Either::Left((Either::Left((None, _)), _)) => break None,

            // messages received from other handlers or server
            // send ServerResponse back to the client
            Either::Left((Either::Right((Some(json_msg), _)), _)) => {
                let possible_response: Result<ServerResponse, _> = from_str(&json_msg);
                match possible_response {
                    Ok(resp) => {
                        println!("Sending response (player_id: {}): {:?}", player_id, resp);
                        resp.send(&mut session).await.unwrap();
                    }
                    Err(_) => { panic!("Invalid ServerResponse received !") }
                }
                // log::info!("chat_msg: {chat_msg:?}");
                // // session.text(serde_json::to_string(&chat_message).unwrap()).await.unwrap();
                // let chat_message = ServerResponse::Message {message: chat_msg};
                // chat_message.send(&mut session).await.unwrap();
            }

            // all connection's message senders were dropped
            Either::Left((Either::Right((None, _)), _)) => unreachable!(
                "all connection message senders were dropped; server may have panicked"
            ),

            // heartbeat internal tick
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    log::info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );
                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        };
    };

    game_server.disconnect(conn_id);

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;
}

/// Process received user data
async fn process_received_text(
    game_server: &GameServerHandle,
    _session: &mut actix_ws::Session,
    text: &str,
    conn: ConnId,
    player_id: PlayerId,
) -> Option<CloseReason> {
    let json_str = text.trim();

    let possible_action: Result<UserAction, _> = from_str(&json_str);

    match possible_action {
        Ok(UserAction::PlayCard { card_index, targets }) => {
            log::info!("Play Card Action: index: {card_index:?}, targets: {targets:?}");
            let _ = game_server.send_play_card_action(player_id, card_index, targets).await;
        },

        Ok(UserAction::DrawCard {  }) => {
            log::info!("Draw Card Action");
            let _ = game_server.send_draw_card_action(player_id).await;
        },

        Ok(UserAction::SendChatMessage { message }) => {
            log::info!("Send Chat Message Action: message: {message:?}");
            game_server.send_message(conn, message).await;
        },

        Err(_) => {
            log::warn!("Unable to deserialize JSON data to a player action: {json_str:?}");
        }
    }

    return None;
}
