use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// structures to keep track of play actions (for server)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayAction {
    pub dice_roll : u8,
    pub player_dice_id: Option<Uuid>,
    pub targets: Vec<ActionTarget>,
}

impl PlayAction {
    pub fn new() -> Self {
        Self { dice_roll: 0, player_dice_id: None, targets: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTarget {
    pub player_id : Uuid,
    pub action: ActionType,
    pub effect: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayInfo {
    pub actions: Vec<PlayAction>
}

impl PlayInfo {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum ActionType {
    Attack{amount: u32},
    Heal{amount: u32},
}