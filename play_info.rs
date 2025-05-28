use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayAction {
    
    pub dice_roll : u8,
    pub targets: Vec<Target>,
    pub player_dice_id: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    
    pub player_id : Uuid,
    pub action: Action,
    pub effect: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayInfo {
    pub actions: Vec<PlayAction>
}

#[serde(tag="type")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Attack{amount: u32},
    Heal{amount: u32},
}
