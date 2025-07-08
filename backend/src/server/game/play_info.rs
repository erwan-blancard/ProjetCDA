use serde_derive::{Deserialize, Serialize};

use crate::server::game::card::CardId;

use super::player::PlayerId;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayAction {
    pub dice_roll : u8,
    pub player_dice_id: PlayerId,
    pub targets: Vec<ActionTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTarget {
    pub player_id : PlayerId,
    pub action: ActionType,
    pub effect: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayInfo {
    pub actions: Vec<PlayAction>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum ActionType {
    Attack{amount: u32},
    Heal{amount: u32},
    Draw{cards: Vec<CardId>},
    Discard{cards: Vec<usize>},
}


impl PlayInfo {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }
}

impl PlayAction {
    pub fn new() -> Self {
        Self { dice_roll: 0, player_dice_id: -1, targets: Vec::new() }
    }
}