use std::time::Duration;

use serde_derive::{Deserialize, Serialize};

use super::cards::card::CardId;
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
    Steal{cards: Vec<CardId>},
}


impl ActionType {
    pub fn get_estimated_time(&self) -> Duration {
        match self {
            // default event duration in web app
            _ => Duration::from_millis(250)
        }
    }
}


impl PlayAction {
    pub fn new() -> Self {
        Self { dice_roll: 0, player_dice_id: -1, targets: Vec::new() }
    }

    pub fn get_estimated_time(&self) -> Duration {
        let time = {
            // if dice roll is used
            if self.dice_roll > 0 {
                Duration::from_millis(120 * 6 + 1000)
            } else { Duration::ZERO }
        };

        time + self.targets.iter()
            .map(|target| target.action.get_estimated_time())
            .sum::<Duration>()
    }
}

impl PlayInfo {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    /// Estimated amount of time that it will take for the web app to show the actions to the user.
    pub fn get_estimated_time(&self) -> Duration {
        self.actions.iter()
            .map(|action| action.get_estimated_time())
            .sum()
    }
}