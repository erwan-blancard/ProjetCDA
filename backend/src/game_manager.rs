use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Messages envoyés **depuis** le client
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    CreateRoom { name: String },
    JoinRoom { room_id: String },
    StartGame,
    PlayCard { card_id: usize },
    DrawCard,
}

/// Messages envoyés **vers** le client
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    RoomCreated { room_id: String },
    JoinedRoom { room_id: String, players: Vec<String> },
    GameStarted { your_hand: Vec<usize>, turn: usize },
    CardPlayed { player: String, card_id: usize },
    CardDrawn  { player: String, card_id: usize },
    Error      { msg: String },
}

/// Session WebSocket pour chaque client
pub struct WsSession {
    pub id: String,
    pub addr: Addr<GameServer>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // prévenir le GameServer de cette nouvelle connexion
        let addr = ctx.address();
        self.addr
            .send(Connect {
                session_id: self.id.clone(),
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|_,_,_| async {})
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { session_id: self.id.clone() });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(txt)) => {
                if let Ok(cmd) = serde_json::from_str::<ClientMessage>(&txt) {
                    self.addr.do_send(ClientWsMessage {
                        session_id: self.id.clone(),
                        msg: cmd,
                    });
                }
            }
            Ok(ws::Message::Ping(b)) => ctx.pong(&b),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => {}
        }
    }
}

/// Connexion d’une session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub session_id: String,
    pub addr: Recipient<ServerMessage>,
}

/// Déconnexion d’une session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: String,
}

/// Message interne quand le client envoie une commande WS
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientWsMessage {
    pub session_id: String,
    pub msg: ClientMessage,
}

/// Serveur principal (Actix Actor)
pub struct GameServer {
    sessions: HashMap<String, Recipient<ServerMessage>>,
    rooms:    HashMap<String, Vec<String>>, // room_id → Vec<session_id>
}

impl GameServer {
    pub fn new() -> Self {
        GameServer {
            sessions: HashMap::new(),
            rooms:    HashMap::new(),
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for GameServer {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.session_id, msg.addr);
    }
}

impl Handler<Disconnect> for GameServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.session_id);
        for players in self.rooms.values_mut() {
            players.retain(|id| id != &msg.session_id);
        }
    }
}

impl Handler<ClientWsMessage> for GameServer {
    type Result = ();
    fn handle(&mut self, msg: ClientWsMessage, _: &mut Context<Self>) {
        let sid = msg.session_id.clone();
        use ClientMessage::*;
        match msg.msg {
            CreateRoom { name: _ } => {
                let room_id = Uuid::new_v4().to_string();
                self.rooms.insert(room_id.clone(), vec![sid.clone()]);
                if let Some(addr) = self.sessions.get(&sid) {
                    let _ = addr.do_send(ServerMessage::RoomCreated { room_id });
                }
            }
            JoinRoom { room_id } => {
                if let Some(players) = self.rooms.get_mut(&room_id) {
                    players.push(sid.clone());
                    for p in players.iter() {
                        if let Some(addr) = self.sessions.get(p) {
                            let _ = addr.do_send(ServerMessage::JoinedRoom {
                                room_id: room_id.clone(),
                                players: players.clone(),
                            });
                        }
                    }
                } else if let Some(addr) = self.sessions.get(&sid) {
                    let _ = addr.do_send(ServerMessage::Error { msg: "Room not found".into() });
                }
            }
            StartGame => {
                // distribue 5 cartes aléatoires [1..100] à chaque joueur
                if let Some((room_id, players)) = self.rooms
                    .iter()
                    .find(|(_, vec)| vec.contains(&sid))
                    .map(|(r, v)| (r.clone(), v.clone()))
                {
                    for p in players {
                        if let Some(addr) = self.sessions.get(&p) {
                            let hand: Vec<usize> = (0..5).map(|_| rand::random::<usize>() % 100 + 1).collect();
                            let _ = addr.do_send(ServerMessage::GameStarted {
                                your_hand: hand,
                                turn: 0,
                            });
                        }
                    }
                }
            }
            PlayCard { card_id } => {
                if let Some((_, players)) = self.rooms
                    .iter()
                    .find(|(_, vec)| vec.contains(&sid))
                    .map(|(r, v)| (r.clone(), v.clone()))
                {
                    for p in players {
                        if let Some(addr) = self.sessions.get(&p) {
                            let _ = addr.do_send(ServerMessage::CardPlayed {
                                player: sid.clone(),
                                card_id,
                            });
                        }
                    }
                }
            }
            DrawCard => {
                let card_id = rand::random::<usize>() % 100 + 1;
                if let Some((_, players)) = self.rooms
                    .iter()
                    .find(|(_, vec)| vec.contains(&sid))
                    .map(|(r, v)| (r.clone(), v.clone()))
                {
                    for p in players {
                        if let Some(addr) = self.sessions.get(&p) {
                            let _ = addr.do_send(ServerMessage::CardDrawn {
                                player: sid.clone(),
                                card_id,
                            });
                        }
                    }
                }
            }
        }
    }
}
