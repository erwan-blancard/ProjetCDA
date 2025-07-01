use super::{card::{Card, EffectId, Element}, game::Game, play_info::{ActionTarget, ActionType}};

const PLAYER_MAX_HEALTH: i32 = 100;

// use uid::Id as IdT;

// #[derive(Copy, Clone, Eq, PartialEq)]
// struct T(());

// pub type PlayerId = IdT<T>;
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
        let mut effective_damage = amount;

        if self.health - i32::try_from(effective_damage).unwrap() < 0 {
            effective_damage = 0;
        } else {
            self.health -= i32::try_from(effective_damage).unwrap();
        }
        
        ActionTarget { player_id: self.id, action: ActionType::Attack{ amount: effective_damage }, effect }   // FIXME effect
    }

    pub fn heal(&mut self, amount: u32, effect: EffectId) -> ActionTarget {
        // TODO check buffs
        let effective_heal = amount;
        if self.health + i32::try_from(effective_heal).unwrap() > PLAYER_MAX_HEALTH {
            self.health = PLAYER_MAX_HEALTH;
        } else {
            self.health += i32::try_from(effective_heal).unwrap();
        }

        ActionTarget { player_id: self.id, action: ActionType::Heal { amount: effective_heal }, effect }   // FIXME effect
    }

}
