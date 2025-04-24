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

use crate::{ConnId, Msg, Player, Token};
use crate::game::engine::GameEngine;
use crate::actions::UserAction;

/// A command received by the [`GameServer`].
#[derive(Debug)]
enum Command {
    Authenticate {
        token: Token,
        conn: ConnId,
        res_tx: oneshot::Sender<bool>,
    },
    
    Connect {
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

    users: HashMap<ConnId, Player>,

    game_engine: GameEngine,

    /// Command receiver.
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl GameServer {
    pub fn new() -> (Self, GameServerHandle) {

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                game_engine: GameEngine::new(),
                users: HashMap::new(),
                cmd_rx,
            },
            GameServerHandle { cmd_tx },
        )
    }

    /// Send user message to others.
    async fn send_chat_message(&self, conn: ConnId, msg: impl Into<Msg>) {
        let msg = msg.into();
        
        for (conn_id, tx) in &self.sessions {
            if conn_id != &conn {
                let _ = tx.send(msg.clone());
            }
        }
    }
    
    /// Authenticate player through the API (TODO)
    async fn authenticate_player(&self, token: Token) -> Option<Player> {
        // TODO
        Some(Player { token, name: String::from("Player") })
    }

    async fn process_player_action(&self, json_string: String, conn_id: ConnId) {

    }

    /// Register new session and assign unique ID to this session
    async fn connect(&mut self, tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        log::info!("Someone joined");

        // notify all users in same room
        self.send_chat_message(0, "Someone joined").await;

        // register session with random connection ID
        let id = rand::rng().random::<ConnId>();
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
                Command::Connect { conn_tx, res_tx } => {
                    let conn_id = self.connect(conn_tx).await;
                    let _ = res_tx.send(conn_id);
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::Authenticate { token, conn, res_tx } => {
                    let mut status = false;
                    if let Some(player) = self.authenticate_player(token).await {
                        self.users.insert(conn, player);
                        status = true;
                    }
                    let _ = res_tx.send(status);
                }

                Command::Action { json_string, conn, res_tx } => {
                    self.process_player_action(json_string, conn).await;
                    let _ = res_tx.send(());
                }

                Command::Message { conn, msg, res_tx } => {
                    self.send_chat_message(conn, msg).await;
                    let _ = res_tx.send(());
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
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: game server should not have been dropped
        self.cmd_tx
            .send(Command::Connect { conn_tx, res_tx })
            .unwrap();

        // unwrap: game server does not drop out response channel
        res_rx.await.unwrap()
    }

    pub async fn authenticate(&self, token: Token, conn: ConnId) -> bool {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: game server should not have been dropped
        self.cmd_tx
            .send(Command::Authenticate { token, conn, res_tx })
            .unwrap();

        // unwrap: game server does not drop out response channel
        res_rx.await.unwrap()
    }

    /// Broadcast message to current room.
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

    pub fn disconnect(&self, conn: ConnId) {
        // unwrap: game server should not have been dropped
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }
}
