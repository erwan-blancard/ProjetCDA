use std::{
    collections::HashMap, io, pin::pin, time::Duration
};

use futures::lock::Mutex;
use futures_util::{
    StreamExt as _, // keep this line
    future::{Either, select},
};

use chrono::Utc;
use tokio::{sync::{mpsc, oneshot}, time::interval};
use uid::IdU64;

use crate::{backend_db::BackendDb, server::{dto::responses::ServerResponse, game::{cards::card::CardId, game::{GameState, DRAW_CARD_LIMIT}, play_info::PlayInfo}}, GameId};

use super::{dto::responses::PlayerProfile, game::{game::Game, player::PlayerId}};


/// Connection ID.
pub type ConnId = u64;

pub type Msg = String;

const TURN_CHECK_INTERVAL: Duration = Duration::from_secs(1);


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


/// Map of connection IDs to the player id and their message receivers.
/// Intended to be wrapped in a Mutex.
#[derive(Debug)]
pub struct SessionsInner {
    sessions: HashMap<ConnId, (PlayerId, mpsc::UnboundedSender<Msg>)>,
}


#[derive(Debug)]
pub struct GameServer {
    sessions: Mutex<SessionsInner>,

    /// list of accounts associated to players
    accounts: Vec<PlayerProfile>,

    game: Game,

    /// Command receiver.
    cmd_rx: mpsc::UnboundedReceiver<Command>,

    /// GameId for this server
    game_id: GameId,

    backend_db: BackendDb,

    /// sent when run is called
    ready_tx: Option<oneshot::Sender<()>>,
}

impl GameServer {
    pub fn new(players: Vec<PlayerProfile>, game_id: GameId, backend_db: BackendDb, ready_tx: oneshot::Sender<()>,) -> (Self, GameServerHandle) {

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        let cards = backend_db.collect_cards().unwrap();

        (
            Self {
                sessions: Mutex::new(SessionsInner { sessions: HashMap::new() }),
                game: Game::new(&players, cards),
                accounts: players,
                cmd_rx,
                game_id,
                backend_db,
                ready_tx: Some(ready_tx),
            },
            GameServerHandle { cmd_tx },
        )
    }

    /// Send user message to others.
    async fn send_chat_message_to_handlers(&self, conn: ConnId, msg: impl Into<Msg>) {
        let msg = msg.into();
        let msg = ServerResponse::Message { message: msg };

        for (conn_id, (_, tx)) in &self.sessions.lock().await.sessions {
            if conn_id != &conn {
                let _ = msg.send_unbounded(tx);
            }
        }
    }

    async fn notify_game_started(&self) {
        for (_, (player_id, tx)) in &self.sessions.lock().await.sessions {
            let status = self.game.status_for_player(*player_id).unwrap();

            let _ = status.to_server_response().send_unbounded(tx);
        }
        
    }

    async fn notify_game_end(&self, winner_id: PlayerId) {
        for (_, (_, tx)) in &self.sessions.lock().await.sessions {
            let resp = ServerResponse::GameEnd { winner_id };
            let _ = resp.send_unbounded(tx);
        }
    }

    async fn send_game_state(&self, player_id: PlayerId) {
        let sessions = &self.sessions.lock().await.sessions;

        let tx = &sessions.iter().find(|(_, (id, _))| *id == player_id).unwrap().1.1;

        let state = self.game.status_for_player(player_id).unwrap();
        let _ = state.to_server_response().send_unbounded(tx);
    }

    async fn advance_turn(&mut self) {
        match self.game.state {
            GameState::EndGame { winner_id } => {
                self.notify_game_end(winner_id).await;
                
                // Reset ready status in the associated lobby
                match self.backend_db.reset_users_ready_on_game_end(&self.game_id) {
                    Err(e) => { log::error!("Error when resetting users ready status on game end: {}", e.to_string()) }
                    _ => {}
                }
                return;
            }
            _ => {}
        }

        self.game.advance_turn();
        self.notify_change_turn().await;

        let current_player_id = self.game.current_player_id();
        let mut card_count = self.game.players[self.game.current_player_turn].hand_cards.len();

        // collect discard cards if needed
        if card_count < DRAW_CARD_LIMIT && self.game.pile.len() < DRAW_CARD_LIMIT - card_count {
            self.game.collect_discard_cards();
            self.game.shuffle_pile();
            let resp = ServerResponse::CollectDiscardCards { cards_in_pile: self.game.pile.len() as u32 };

            for (_, (_, tx)) in &self.sessions.lock().await.sessions {
                let _ = resp.send_unbounded(tx);
            }
        }

        // draw cards if player has less than 5 cards
        while card_count < DRAW_CARD_LIMIT {
            let card_id = self.game.draw_card(current_player_id).unwrap();
            card_count += 1;

            for (_, (pid, tx)) in &self.sessions.lock().await.sessions {
                let resp = ServerResponse::DrawCard {
                    player_id: current_player_id,
                    card_id: if *pid == current_player_id { card_id } else { -1 },
                };
                let _ = resp.send_unbounded(tx);
            }
        }
    }

    async fn notify_change_turn(&self) {
        let resp = ServerResponse::ChangeTurn { player_id: self.game.current_player_id(), turn_end: self.game.current_player_turn_end };
        for (_, (_, tx)) in &self.sessions.lock().await.sessions {
            let _ = resp.send_unbounded(tx);
        }
    }

    /// Register new session and assign unique ID to this session
    async fn connect(&mut self, player_id: PlayerId, tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        log::info!("Someone joined");

        // stop connection associated with this player id (if any)
        let maybe_conn_id = {
            let sessions = &self.sessions.lock().await.sessions;
            sessions.iter().find(|(_, (id, _))| *id == player_id).map(|(id, _)| *id)
        };

        if let Some(conn_id) = maybe_conn_id {
            self.disconnect(conn_id).await;
        }

        // notify all users in same session
        self.send_chat_message_to_handlers(0, "Someone joined").await;

        // register session with random connection ID
        let id = IdU64::<ConnId>::new().get();

        {
            let sessions = &mut self.sessions.lock().await.sessions;
            sessions.insert(id, (player_id, tx));
        }

        // send id back
        id
    }

    /// Unregister connection from room map and broadcast disconnection message.
    async fn disconnect(&mut self, conn_id: ConnId) {
        let sessions = &mut self.sessions.lock().await.sessions;

        // remove sender
        if sessions.remove(&conn_id).is_some() {
            println!("Session {conn_id:?} disconnected");
            // extra stuff
        }
    }

    pub async fn run(mut self) -> io::Result<()> {
        if let Some(ready_tx) = self.ready_tx.take() {
            let _ = ready_tx.send(());
        }

        // interval used to check if a player misses a turn
        let mut interval = interval(TURN_CHECK_INTERVAL);
        
        // Extract the receiver from self to avoid borrow conflicts
        // FIXME: this is a hack to avoid borrow conflicts, self.cmd_rx is replace with a new, useless one
        let mut cmd_rx = std::mem::replace(&mut self.cmd_rx, mpsc::unbounded_channel().1);

        loop {
            // check if game is over
            match self.game.state {
                GameState::EndGame { .. } => { break; } // exit loop to stop the server
                _ => {}
            }

            let tick = pin!(interval.tick());
            let msg_rx = pin!(cmd_rx.recv());

            match select(msg_rx, tick).await {

                Either::Left((Some(cmd), _)) => {
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

                                let resp = ServerResponse::PlayCard {
                                    player_id,
                                    card_id: card_id.unwrap_or(-1),
                                    hand_index: card_index as u32,
                                    actions: play_info.actions.clone(),
                                };
                                // list of buffs of the player
                                let buffs_resp = ServerResponse::PlayerBuffStatus {
                                    player_id,
                                    buffs: self.game.players[self.game.current_player_turn]
                                        .buffs.iter()
                                        .map(|b| b.as_variant())
                                        .collect()
                                };
                                // send responses to clients
                                for (_, (_, tx)) in &self.sessions.lock().await.sessions {
                                    let _ = resp.send_unbounded(tx);
                                    let _ = buffs_resp.send_unbounded(tx);
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
                                for (_, (pid, tx)) in &self.sessions.lock().await.sessions {
                                    let resp = ServerResponse::DrawCard {
                                        player_id,
                                        card_id: if *pid == player_id { card_id } else { -1 },
                                    };
                                    let _ = resp.send_unbounded(tx);
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
                            let _ = res_tx.send(());
                            // exit loop
                            break;
                        }
                    }
                }

                // cmd_rx is closed
                Either::Left((None, _)) => {
                    break;
                }

                Either::Right((_, _tick)) => {
                    // check if current player missed his turn
                    match self.game.state {
                        GameState::InGame => {
                            if self.game.current_player_turn_end < Utc::now() {
                                self.advance_turn().await;
                            }
                        }
                        _ => {}
                    }
                }

            }
        }

        {
            let sessions = &mut self.sessions.lock().await.sessions;
            sessions.clear();
        }

        if !cmd_rx.is_closed() {
            cmd_rx.close();
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
