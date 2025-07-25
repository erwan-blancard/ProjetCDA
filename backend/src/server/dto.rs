use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::GameId;

pub mod actions;
pub mod responses;

use self::responses::PlayerProfile;


#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GameSessionInfo {
    #[schema(value_type = String)]
    pub game_id: GameId,
    pub players: Vec<PlayerProfile>,
}
