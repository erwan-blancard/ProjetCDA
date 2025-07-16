use actix_ws::{Session, Closed};
use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;

use crate::server::game::buffs::{Buff, BuffVariant};
use crate::server::game::cards::card::CardId;
use crate::server::game::play_info::PlayAction;
use crate::server::game::player::PlayerId;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerProfile {
    pub id: PlayerId,
    pub name: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpponentState {
    pub player_id: PlayerId,
    pub health: u32,
    pub card_count: u32,
    pub discard_cards: Vec<CardId>,
    pub buffs: Vec<BuffVariant>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GameStateForPlayer {
    pub current_player_turn: PlayerId,
    #[serde(with = "ts_seconds")]   // needed to serialize a DateTime with serde
    pub current_player_turn_end: DateTime<Utc>,
    pub health: u32,
    pub cards: Vec<CardId>,
    pub discard_cards: Vec<CardId>,
    pub buffs: Vec<BuffVariant>,
    pub opponents: Vec<OpponentState>,
    pub cards_in_pile: u32
}

impl GameStateForPlayer {
    pub fn to_server_response(&self) -> ServerResponse {
        ServerResponse::GameStatus {
            current_player_turn: self.current_player_turn,
            current_player_turn_end: self.current_player_turn_end,
            health: self.health,
            cards: self.cards.clone(),
            discard_cards: self.discard_cards.clone(),
            buffs: self.buffs.clone(),
            opponents: self.opponents.clone(),
            cards_in_pile: self.cards_in_pile
        }
    }
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
        buffs: Vec<BuffVariant>,
        opponents: Vec<OpponentState>,
        cards_in_pile: u32
    },
    
    // Game Actions

    PlayCard {
        player_id: PlayerId,
        card_id: CardId,
        /// Index of card in player's hand
        hand_index: u32,
        actions: Vec<PlayAction>,
    },

    /// a card was drawn by a player
    DrawCard {
        player_id: PlayerId,
        /// -1 if opponent card
        card_id: CardId
    },

    /// notify client of turn change
    ChangeTurn {
        player_id: PlayerId,
        #[serde(with = "ts_seconds")]   // needed to serialize a DateTime with serde
        turn_end: DateTime<Utc>
    },

    /// notify client that all discarded cards were collected to pile
    CollectDiscardCards {
        cards_in_pile: u32,
    },

    /// send list of active buffs for the player
    PlayerBuffStatus {
        player_id: PlayerId,
        buffs: Vec<BuffVariant>
    },

    /// notify client of game end
    GameEnd {
        winner_id: PlayerId
    }
}

impl ServerResponse {
    /// send as text through the Session
    pub async fn send(&self, session: &mut Session) -> Result<(), Closed> {
        session.text(serde_json::to_string(&self).unwrap()).await
    }

    pub fn send_unbounded(&self, tx: &UnboundedSender<String>) -> Result<(), SendError<String>> {
        tx.send(serde_json::to_string(&self).unwrap())
    }
}
