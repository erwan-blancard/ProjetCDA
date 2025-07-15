use super::{card::{Card, EffectId, Element}, play_info::{ActionTarget, ActionType}};

const PLAYER_MAX_HEALTH: i32 = 100;


pub type PlayerId = i32;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub health: i32,
    pub hand_cards: Vec<Box<dyn Card>>,
    pub discard_cards: Vec<Box<dyn Card>>,
    pub casted_cards: Vec<Box<dyn Card>>,
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
            casted_cards: Vec::new(),
        }
    }

    pub fn damage(&mut self, amount: u32, element: Element, effect: EffectId) -> ActionTarget {
        // TODO check buffs
        // TODO check element
        let effective_damage = amount as i32;

        if self.health - effective_damage < 0 {
            self.health = 0;
        } else {
            self.health -= effective_damage;
        }
        
        ActionTarget { player_id: self.id, action: ActionType::Attack{ amount: effective_damage as u32 }, effect }   // FIXME effect
    }

    pub fn heal(&mut self, amount: u32, effect: EffectId) -> ActionTarget {
        // TODO check buffs
        let effective_heal = amount as i32;
        if self.health + effective_heal > PLAYER_MAX_HEALTH {
            self.health = PLAYER_MAX_HEALTH;
        } else {
            self.health += effective_heal;
        }

        ActionTarget { player_id: self.id, action: ActionType::Heal { amount: effective_heal as u32 }, effect }   // FIXME effect
    }

}
