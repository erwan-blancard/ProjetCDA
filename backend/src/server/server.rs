use std::{
    collections::{HashMap, HashSet},
    io,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use chrono::{DateTime, Utc};
use rand::{rand_core::le, Rng as _};
use serde_json::to_string;
use tokio::sync::{mpsc, oneshot};
use uid::IdU64;

use crate::{database::models::Account, routes::game::{Lobbies, Lobby, LobbyId}, server::{dto::responses::ServerResponse, game::{card::CardId, game::{GameState, DRAW_CARD_LIMIT}, play_info::PlayInfo}}, GameId};

use super::{dto::responses::{GameStateForPlayer, PlayerProfile}, game::{game::Game, player::{Player, PlayerId}}};
use super::dto::actions::UserAction;


/// Connection ID.
pub type ConnId = u64;

pub type Msg = String;
pub type Token = String;


/// A command received by the [`GameServer`] (sent by a [`GameServerHandle`])
#[derive(Debug)]
enum Command {
    Connect {
        player_id: PlayerId,
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<ConnId>,
    },

    Disconnect {
        conn: ConnId,
    },

    SessionInfo {
        res_tx: oneshot::Sender<Vec<PlayerProfile>>,
    },

    /// send game state to client
    GameStateForPlayer {
        player_id: PlayerId,
        res_tx: oneshot::Sender<()>,
    },

    PlayCard {
        player_id: PlayerId,
        card_index: usize,
        targets: Vec<PlayerId>,
        res_tx: oneshot::Sender<Result<PlayInfo, String>>,
    },

    DrawCard {
        player_id: PlayerId,
        res_tx: oneshot::Sender<Result<CardId, String>>,
    },

    Message {
        msg: Msg,
        conn: ConnId,
        res_tx: oneshot::Sender<()>,
    },

    Kill {
        res_tx: oneshot::Sender<()>,
    }
}


#[derive(Debug)]
pub struct GameServer {
    /// Map of connection IDs to their message receivers.
    sessions: HashMap<ConnId, mpsc::UnboundedSender<Msg>>,

    /// list of accounts associated to players
    accounts: Vec<PlayerProfile>,

    /// users must be authenticated when in this map
    users: HashMap<PlayerId, ConnId>,

    game: Game,

    /// Command receiver.
    cmd_rx: mpsc::UnboundedReceiver<Command>,

    /// GameId for this server
    game_id: GameId,

    /// Shared lobbies handle
    lobbies: Lobbies,

    /// sent when run is called
    ready_tx: Option<oneshot::Sender<()>>,
}

impl GameServer {
    pub fn new(players: Vec<PlayerProfile>, game_id: GameId, lobbies: Lobbies, ready_tx: oneshot::Sender<()>,) -> (Self, GameServerHandle) {

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                game: Game::new(&players),
                accounts: players,
                users: HashMap::new(),
                cmd_rx,
                game_id,
                lobbies,
                ready_tx: Some(ready_tx),
            },
            GameServerHandle { cmd_tx },
        )
    }

    /// Send user message to others.
    async fn send_chat_message_to_handlers(&self, conn: ConnId, msg: impl Into<Msg>) {
        let msg = msg.into();
        let msg = ServerResponse::Message { message: msg };

        for (conn_id, tx) in &self.sessions {
            if conn_id != &conn {
                let _ = msg.send_unbounded(tx);
            }
        }
    }

    async fn notify_game_started(&self) {
        for (player_id, conn_id) in self.users.clone() {
            let tx = self.sessions.get(&conn_id).unwrap();
            let status = self.game.status_for_player(player_id).unwrap();

            let _ = status.to_server_response().send_unbounded(tx);
        }
    }

    async fn notify_game_end(&self, winner_id: PlayerId) {
        for (_, conn_id) in self.users.clone() {
            let tx = self.sessions.get(&conn_id).unwrap();
            let resp = ServerResponse::GameEnd { winner_id };
            let _ = resp.send_unbounded(tx);
        }
    }

    async fn send_game_state(&self, player_id: PlayerId) {
        let state = self.game.status_for_player(player_id).unwrap();
        let conn_id = self.users.get(&player_id).unwrap();
        let _ = state.to_server_response().send_unbounded(self.sessions.get(&conn_id).unwrap());
    }

    async fn advance_turn(&mut self) {
        match self.game.state {
            GameState::EndGame { winner_id } => {
                self.notify_game_end(winner_id).await;

                self.sessions.clear();

                // close handler channel
                self.cmd_rx.close();
                
                // Reset ready status in the associated lobby
                {
                    let mut lobbies = self.lobbies.lock().unwrap();
                    let maybe_lobby = lobbies.iter_mut().find(|(_, lobby)| lobby.game_id == Some(self.game_id));
                    if let Some((_lobby_id, lobby)) = maybe_lobby {
                        lobby.users_ready.clear();
                    }
                }
                return;
            }
            _ => {}
        }

        self.game.current_player_turn = self.game.next_player_index();
        self.notify_change_turn().await;

        let current_player_id = self.game.current_player_id();
        let mut card_count = self.game.players[self.game.current_player_turn].hand_cards.len();

        println!("player card count: {}, cards in pile: {}, condition: {}", card_count, self.game.pile.len(), card_count < DRAW_CARD_LIMIT && self.game.pile.len() < DRAW_CARD_LIMIT - card_count);

        // collect discard cards if needed
        if card_count < DRAW_CARD_LIMIT && self.game.pile.len() < DRAW_CARD_LIMIT - card_count {
            self.game.collect_discard_cards();
            self.game.shuffle_pile();
            let resp = ServerResponse::CollectDiscardCards { cards_in_pile: self.game.pile.len() as u32 };
            for (_, conn_id) in self.users.clone() {
                let tx = self.sessions.get(&conn_id).unwrap();
                let _ = resp.send_unbounded(tx);
            }
        }

        // draw cards if player has less than 5 cards
        while card_count < DRAW_CARD_LIMIT {
            let card_id = self.game.draw_card(current_player_id).unwrap();
            card_count += 1;
            for (&pid, &conn_id) in &self.users {
                if let Some(tx) = self.sessions.get(&conn_id) {
                    let resp = ServerResponse::DrawCard {
                        player_id: current_player_id,
                        card_id: if pid == current_player_id { card_id } else { -1 },
                    };
                    let _ = resp.send_unbounded(tx);
                }
            }
        }
    }

    async fn notify_change_turn(&self) {
        let resp = ServerResponse::ChangeTurn { player_id: self.game.current_player_id() };
        for (_, conn_id) in self.users.clone() {
            let tx = self.sessions.get(&conn_id).unwrap();

            let _ = resp.send_unbounded(tx);
        }
    }

    /// Register new session and assign unique ID to this session
    async fn connect(&mut self, player_id: PlayerId, tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        log::info!("Someone joined");

        // stop connection associated with this player id (if any)
        if let Some(conn_id) = self.users.get(&player_id) {
            self.disconnect(*conn_id).await;
        }

        // notify all users in same session
        self.send_chat_message_to_handlers(0, "Someone joined").await;

        // register session with random connection ID
        let id = IdU64::<ConnId>::new().get();
        self.sessions.insert(id, tx);
        self.users.insert(player_id, id);

        // send id back
        id
    }

    /// Unregister connection from room map and broadcast disconnection message.
    async fn disconnect(&mut self, conn_id: ConnId) {

        // remove sender
        if self.sessions.remove(&conn_id).is_some() {
            println!("Session {conn_id:?} disconnected");
            // extra stuff
        }
    }

    pub async fn run(mut self) -> io::Result<()> {
        if let Some(ready_tx) = self.ready_tx.take() {
            let _ = ready_tx.send(());
        }

        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect { player_id, conn_tx, res_tx } => {
                    let conn_id = self.connect(player_id, conn_tx).await;
                    let _ = res_tx.send(conn_id);

                    match self.game.state {
                        // not yet started
                        GameState::PreGame => {
                            self.game.begin();
                            self.notify_game_started().await;
                        }
                        GameState::InGame => {
                            self.send_game_state(player_id).await;
                        }
                        // finished
                        // should not happen as we exit the recv loop
                        GameState::EndGame { winner_id: _ } => {
                            self.disconnect(conn_id).await;
                            // exit loop
                            break;
                        }
                    }
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::SessionInfo { res_tx } => {
                    let players = self.game.player_profiles.clone();
                    let _ = res_tx.send(players);
                }

                Command::PlayCard { player_id, card_index, targets, res_tx } => {
                    // match self.game.state {
                    //     // should not happen as we exit the recv loop
                    //     GameState::EndGame { .. } => {
                    //         let _ = res_tx.send(Err("Game is over".to_string()));
                    //         // exit loop
                    //         break;
                    //     }
                    //     _ => {}
                    // }

                    // get card id before it is removed from hand
                    let card_id = self.game.players
                        .iter()
                        .find(|p| p.id == player_id)
                        .and_then(|p| p.hand_cards.get(card_index))
                        .map(|c| c.get_id());

                    let result = self.game.play_card(player_id, card_index, targets.clone());
                    let ok = result.is_ok();
                    let _ = res_tx.send(result.clone());

                    if ok {
                        let play_info = result.unwrap();
                        for (&_, &conn_id) in &self.users {
                            if let Some(tx) = self.sessions.get(&conn_id) {
                                let resp = ServerResponse::PlayCard {
                                    player_id,
                                    card_id: card_id.unwrap_or(-1),
                                    hand_index: card_index as u32,
                                    actions: play_info.actions.clone(),
                                };
                                let _ = resp.send_unbounded(tx);
                            }
                        }
                        self.advance_turn().await;
                    } else {
                        // send game state to player when error
                        println!("Error playing card: {:?}", result.clone().err().unwrap());
                        self.send_game_state(player_id).await;
                    }
                }

                Command::DrawCard { player_id, res_tx } => {
                    // match self.game.state {
                    //     // should not happen as we exit the recv loop
                    //     GameState::EndGame { .. } => {
                    //         let _ = res_tx.send(Err("Game is over".to_string()));
                    //         // exit loop
                    //         break;
                    //     }
                    //     _ => {}
                    // }
                    
                    let result = self.game.draw_card(player_id);
                    let ok = result.is_ok();
                    let card_id = result.clone().unwrap_or(-1);
                    let _ = res_tx.send(result.clone());

                    if ok {
                        for (&pid, &conn_id) in &self.users {
                            if let Some(tx) = self.sessions.get(&conn_id) {
                                let resp = ServerResponse::DrawCard {
                                    player_id,
                                    card_id: if pid == player_id { card_id } else { -1 },
                                };
                                let _ = resp.send_unbounded(tx);
                            }
                        }
                        self.advance_turn().await;
                    } else {
                        // send game state to player when error
                        println!("Error drawing card: {:?}", result.clone().err().unwrap());
                        self.send_game_state(player_id).await;
                    }
                }

                Command::Message { conn, msg, res_tx } => {
                    self.send_chat_message_to_handlers(conn, msg).await;
                    let _ = res_tx.send(());
                }

                Command::GameStateForPlayer { player_id, res_tx } => {
                    self.send_game_state(player_id).await;
                    let _ = res_tx.send(());
                }

                Command::Kill { res_tx } => {
                    log::info!("Received kill command");
                    self.sessions.clear();
                    let _ = res_tx.send(());
                    self.cmd_rx.close();
                    // exit loop
                    break;
                }
            }

            match self.game.state {
                GameState::EndGame { .. } => { break; } // exit loop to stop the server
                _ => {}
            }

        }

        log::info!("GameServer worker stopped (game ended)");
        
        Ok(())
    }
}


/// Handle and command sender for game server.
#[derive(Debug, Clone)]
pub struct GameServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl GameServerHandle {
    /// Register client message sender and obtain connection ID.
    pub async fn connect(&self, player_id: PlayerId, conn_tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: game server should not have been dropped
        self.cmd_tx
            .send(Command::Connect { player_id, conn_tx, res_tx })
            .unwrap();

        // unwrap: game server does not drop out response channel
        res_rx.await.unwrap()
    }

    /// Broadcast message to users.
    pub async fn send_message(&self, conn: ConnId, msg: impl Into<Msg>) {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: game server should not have been dropped
        self.cmd_tx
            .send(Command::Message {
                msg: msg.into(),
                conn,
                res_tx,
            })
            .unwrap();

        // unwrap: game server does not drop our response channel
        res_rx.await.unwrap();
    }

    pub async fn get_session_info(&self) -> Vec<PlayerProfile> {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: game server should not have been dropped
        self.cmd_tx
            .send(Command::SessionInfo {
                res_tx,
            })
            .unwrap();

        // unwrap: game server does not drop our response channel
        res_rx.await.unwrap()
    }

    pub async fn send_game_state_for_player(&self, player_id: PlayerId) {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: game server should not have been dropped
        self.cmd_tx
            .send(Command::GameStateForPlayer {
                player_id, res_tx
            })
            .unwrap();

        // unwrap: game server does not drop our response channel
        let _ = res_rx.await.unwrap();
    }

    pub async fn send_play_card_action(&self, player_id: PlayerId, card_index: usize, targets: Vec<PlayerId>) -> Result<PlayInfo, String> {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: game server should not have been dropped
        self.cmd_tx
            .send(Command::PlayCard {
                player_id,
                card_index,
                targets,
                res_tx,
            })
            .unwrap();

        // unwrap: game server does not drop our response channel
        res_rx.await.unwrap()
    }

    pub async fn send_draw_card_action(&self, player_id: PlayerId) -> Result<CardId, String> {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: game server should not have been dropped
        self.cmd_tx
            .send(Command::DrawCard {
                player_id,
                res_tx,
            })
            .unwrap();

        // unwrap: game server does not drop our response channel
        res_rx.await.unwrap()
    }

    pub fn disconnect(&self, conn: ConnId) {
        // unwrap: game server should not have been dropped
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }

    pub fn is_closed(&self) -> bool {
        self.cmd_tx.is_closed()
    }

    pub async fn kill_server(&self) {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx.send(Command::Kill { res_tx }).unwrap();
        res_rx.await.unwrap();
    }
}
