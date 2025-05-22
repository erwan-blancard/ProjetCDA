use actix_ws::{Session, Closed};
use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;


// TEMP
pub type PlayerId = u8;
pub type CardId = u32;


#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerProfile {
    id: PlayerId,
    name: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OpponentState {
    player_id: PlayerId,
    health: u32,
    card_count: u32,
    discard_cards: Vec<CardId>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GameStateForPlayer {
    player_id: PlayerId,
    current_player_turn: PlayerId,
    #[serde(with = "ts_seconds")]   // needed to serialize a DateTime with serde
    current_player_turn_end: DateTime<Utc>,
    health: u32,
    cards: Vec<CardId>,
    discard_cards: Vec<CardId>,
    opponents: Vec<OpponentState>
}


/// JSON structures for server responses
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerResponse {
    Auth {status: bool},
    Message {message: String},
    SessionInfo {
        id: PlayerId,       // which id is assigned to the user
        players: Vec<PlayerProfile>     // all players info
    },
    GameStatus {state: GameStateForPlayer}   // TODO
}

impl ServerResponse {
    /// send as text through the Session
    pub async fn send(&self, session: &mut Session) -> Result<(), Closed> {
        session.text(serde_json::to_string(&self).unwrap()).await
    }
}
