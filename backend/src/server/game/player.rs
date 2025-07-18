use crate::server::game::buffs::Buff;

use super::{cards::card::{Card, EffectId}, play_info::{ActionTarget, ActionType}};

use rand::Rng;


const PLAYER_MAX_HEALTH: i32 = 100;


pub type PlayerId = i32;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub health: i32,
    pub hand_cards: Vec<Box<dyn Card>>,
    pub discard_cards: Vec<Box<dyn Card>>,
    pub buffs: Vec<Box<dyn Buff>>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool { self == other }
    fn ne(&self, other: &Self) -> bool { self != other }
}

impl Player {
    pub fn new(id: PlayerId, name: String) -> Self {
        Self {
            id,
            name,
            health: PLAYER_MAX_HEALTH,
            hand_cards: Vec::new(),
            discard_cards: Vec::new(),
            buffs: Vec::new(),
        }
    }

    pub fn damage(&mut self, amount: u32, effect: EffectId) -> ActionTarget {
        let effective_damage = amount as i32;

        if self.health - effective_damage < 0 {
            self.health = 0;
        } else {
            self.health -= effective_damage;
        }
        
        ActionTarget { player_id: self.id, action: ActionType::Attack{ amount: effective_damage as u32 }, effect }
    }

    pub fn heal(&mut self, amount: u32, effect: EffectId) -> ActionTarget {
        let effective_heal = amount as i32;
        if self.health + effective_heal > PLAYER_MAX_HEALTH {
            self.health = PLAYER_MAX_HEALTH;
        } else {
            self.health += effective_heal;
        }

        ActionTarget { player_id: self.id, action: ActionType::Heal { amount: effective_heal as u32 }, effect }
    }

    pub fn remove_random_card(&mut self) -> Option<Box<dyn Card>> {
        if self.hand_cards.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.hand_cards.len());
        Some(self.hand_cards.remove(idx))
    }
}
