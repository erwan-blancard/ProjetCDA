use actix_ws::{Session, Closed};
use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

use crate::server::game::card::{CardId, EffectId};
use crate::server::game::player::PlayerId;



#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerProfile {
    pub id: PlayerId,
    pub name: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OpponentState {
    player_id: PlayerId,
    health: u32,
    card_count: u32,
    discard_cards: Vec<CardId>,
}


#[derive(Serialize, Deserialize, Debug)]
/// Struct of a card action. A card may do more than one action when played.
pub struct CardAction {
    dice_roll: u8,
    targets: Vec<CardActionTarget>
}


#[derive(Serialize, Deserialize, Debug)]
/// Describes a card action target
pub struct CardActionTarget {
    player_id: PlayerId,
    action: CardActionType,
    effect: EffectId,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CardActionType {
    Attack {amount: u32},
    Heal {amount: u32},

}


#[derive(Serialize, Deserialize, Debug)]
pub struct GameStateForPlayer {
    current_player_turn: PlayerId,
    #[serde(with = "ts_seconds")]   // needed to serialize a DateTime with serde
    current_player_turn_end: DateTime<Utc>,
    health: u32,
    cards: Vec<CardId>,
    discard_cards: Vec<CardId>,
    opponents: Vec<OpponentState>,
    cards_in_pile: u32
}


/// JSON structures for server responses
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerResponse {
    Message {message: String},
    /// Sent when player connects to WebSocket
    SessionInfo {
        /// which player id is assigned to the client
        id: PlayerId,
        /// all players info
        players: Vec<PlayerProfile>
    },

    /// Game Status (personnalised for each client)
    GameStatus {
        current_player_turn: PlayerId,
        #[serde(with = "ts_seconds")]   // needed to serialize a DateTime with serde
        current_player_turn_end: DateTime<Utc>,
        health: u32,
        cards: Vec<CardId>,
        discard_cards: Vec<CardId>,
        opponents: Vec<OpponentState>,
        cards_in_pile: u32
    },
    
    // Game Actions

    PlayCard {
        player_id: PlayerId,
        card_id: CardId,
        /// Index of card in player's hand
        hand_index: u32,
        targets: Vec<CardActionTarget>,
    },

    /// a card was drawn by a player
    DrawCard {
        player_id: PlayerId,
        /// -1 if opponent card
        card_id: CardId
    },

    /// notify client of turn change
    ChangeTurn {
        player_id: PlayerId
    },
}

impl ServerResponse {
    /// send as text through the Session
    pub async fn send(&self, session: &mut Session) -> Result<(), Closed> {
        session.text(serde_json::to_string(&self).unwrap()).await
    }
}
