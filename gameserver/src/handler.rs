use std::{
    pin::pin, str::FromStr, time::{Duration, Instant}
};

use actix_ws::{AggregatedMessage, CloseReason, CloseCode};
use futures_util::{
    StreamExt as _,
    future::{Either, select},
};
use serde_json::{from_str, Value};
use tokio::{sync::mpsc, time::interval};

use crate::{actions::UserAction, game, ConnId, GameServerHandle};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// How long a connection can last unauthenticated
const AUTHENTICATION_TIMEOUT: Duration = Duration::from_secs(15);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn game_ws(
    game_server: GameServerHandle,
    mut session: actix_ws::Session,
    msg_stream: actix_ws::MessageStream,
) {
    log::info!("connected");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    // unwrap: server is not dropped before the HTTP server
    let conn_id = game_server.connect(conn_tx).await;

    let msg_stream = msg_stream
        .max_frame_size(128 * 1024)
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);

    let mut msg_stream = pin!(msg_stream);

    let mut authenticated = false;
    let auth_wait_begin = Instant::now();

    let close_reason = loop {
        // most of the futures we process need to be stack-pinned to work with select()

        let tick = pin!(interval.tick());
        let msg_rx = pin!(conn_rx.recv());

        if !authenticated && Instant::now().duration_since(auth_wait_begin) > AUTHENTICATION_TIMEOUT {
            let desc = String::from("Authentication Timeout reached !");
            log::warn!("{:?}", desc);
            break Some(CloseReason { code: CloseCode::Normal, description: Some(desc) });
        }

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
                        let close_reason = process_received_text(&game_server, &mut session, &text, conn_id, &mut authenticated).await;
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

            // chat messages received from other room participants
            Either::Left((Either::Right((Some(chat_msg), _)), _)) => {
                session.text(chat_msg).await.unwrap();
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
    session: &mut actix_ws::Session,
    text: &str,
    conn: ConnId,
    authenticated: &mut bool
) -> Option<CloseReason> {
    let json_str = text.trim();

    let possible_action: Result<UserAction, _> = from_str(&json_str);

    if !*authenticated {
        match possible_action {
            // actions allowed when not connected
            Ok(UserAction::Auth {..}) => {},
            Err(_) => {
                log::warn!("Unable to deserialize JSON data to a player action: {json_str:?}");
                return None;
            },
            // else the session is closed
            _ => {
                log::warn!("Received player action but the player is not authenticated ! Closing connection...");

                game_server.disconnect(conn);
                
                return Some(
                    CloseReason {
                        code: CloseCode::Normal,
                        description: Some(String::from("Connection closed because the session was not authenticated !"))
                    });
            }
        }
    }

    match possible_action {
        Ok(UserAction::Auth { token }) => {
            log::info!("Auth Action: token: {token:?}");
            let ok = game_server.authenticate(token, conn).await;
            *authenticated = ok;
        },

        Ok(UserAction::PlayCard { card_id }) => {
            log::info!("Play Card Action: id: {card_id:?}");
        },

        Ok(UserAction::DrawCard {  }) => {
            log::info!("Draw Card Action");
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
