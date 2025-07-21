use serde_derive::{Deserialize, Serialize};

use crate::GameId;

pub mod actions;
pub mod responses;

use self::responses::PlayerProfile;


#[derive(Debug, Deserialize, Serialize)]
pub struct GameSessionInfo {
    pub game_id: GameId,
    pub players: Vec<PlayerProfile>,
}
