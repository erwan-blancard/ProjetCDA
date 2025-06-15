use serde_derive::{Deserialize, Serialize};

use crate::{server::dto::responses::PlayerProfile, GameId};


#[derive(Debug, Deserialize, Serialize)]
pub struct GameSessionInfo {
    pub game_id: GameId,
    pub players: Vec<PlayerProfile>,
}
