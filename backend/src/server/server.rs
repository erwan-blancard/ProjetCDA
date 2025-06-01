use std::{
    collections::{HashMap, HashSet},
    io,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use rand::{rand_core::le, Rng as _};
use tokio::sync::{mpsc, oneshot};
use uid::IdU64;

use crate::database::models::Account;

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

    Action {
        json_string: Msg,
        conn: ConnId,
        res_tx: oneshot::Sender<()>,
    },

    SessionInfo {
        res_tx: oneshot::Sender<Vec<PlayerProfile>>,
    },

    GameStateForPlayer {
        conn: ConnId,
        res_tx: oneshot::Sender<Option<GameStateForPlayer>>,
    },

    Message {
        msg: Msg,
        conn: ConnId,
        res_tx: oneshot::Sender<()>,
    },
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
}

impl GameServer {
    pub fn new(players: Vec<PlayerProfile>) -> (Self, GameServerHandle) {

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                game: Game::new(&players),
                accounts: players,
                users: HashMap::new(),
                cmd_rx,
            },
            GameServerHandle { cmd_tx },
        )
    }

    /// Send user message to others.
    async fn send_chat_message_to_handlers(&self, conn: ConnId, msg: impl Into<Msg>) {
        let msg = msg.into();
        
        for (conn_id, tx) in &self.sessions {
            if conn_id != &conn {
                let _ = tx.send(msg.clone());
            }
        }
    }

    async fn process_player_action(&self, json_string: String, conn_id: ConnId) {

    }

    async fn get_game_state_for_player(&self, conn: ConnId) -> Option<GameStateForPlayer> {
        None
    }

    /// Register new session and assign unique ID to this session
    async fn connect(&mut self, player_id: PlayerId, tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        log::info!("Someone joined");

        // stop connection associated with this player id (if any)
        if let Some(conn_id) = self.users.get(&player_id) {
            self.disconnect(*conn_id);
        }

        // notify all users in same session
        self.send_chat_message_to_handlers(0, "Someone joined").await;

        // register session with random connection ID
        let id = IdU64::<ConnId>::new().get();
        self.sessions.insert(id, tx);

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
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect { player_id, conn_tx, res_tx } => {
                    let conn_id = self.connect(player_id, conn_tx).await;
                    let _ = res_tx.send(conn_id);
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::SessionInfo { res_tx } => {
                    // FIXME get from GameEngine
                    let players = Vec::new();

                    let _ = res_tx.send(players);
                }

                Command::Action { json_string, conn, res_tx } => {
                    self.process_player_action(json_string, conn).await;
                    let _ = res_tx.send(());
                }

                Command::Message { conn, msg, res_tx } => {
                    self.send_chat_message_to_handlers(conn, msg).await;
                    let _ = res_tx.send(());
                }

                Command::GameStateForPlayer { conn, res_tx } => {
                    let game_state: Option<GameStateForPlayer> = self.get_game_state_for_player(conn).await;
                    let _ = res_tx.send(game_state);
                }
            }
        }

        Ok(())
    }
}


/// Handle and command sender for game server.
///
/// Reduces boilerplate of setting up response channels in WebSocket handlers.
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

    pub async fn send_game_state_to_player(&self, conn: ConnId) {

    }

    pub fn disconnect(&self, conn: ConnId) {
        // unwrap: game server should not have been dropped
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }

    pub fn is_closed(&self) -> bool {
        self.cmd_tx.is_closed()
    }
}
