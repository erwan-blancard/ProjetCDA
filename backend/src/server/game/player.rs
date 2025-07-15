use super::{card::{Card, EffectId, Element}, play_info::{ActionTarget, ActionType}};
use rand::Rng;

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

    /// Vole une carte spécifique (par id) à un autre joueur. Retourne true si succès.
    pub fn steal_card(&mut self, from: &mut Player, card_id: crate::server::game::card::CardId) -> bool {
        if let Some(pos) = from.hand_cards.iter().position(|c| c.get_id() == card_id) {
            let card = from.hand_cards.remove(pos);
            self.hand_cards.push(card);
            true
        } else {
            false
        }
    }

    /// Donne une carte spécifique (par id) à un autre joueur. Retourne true si succès.
    pub fn give_card(&mut self, to: &mut Player, card_id: crate::server::game::card::CardId) -> bool {
        if let Some(pos) = self.hand_cards.iter().position(|c| c.get_id() == card_id) {
            let card = self.hand_cards.remove(pos);
            to.hand_cards.push(card);
            true
        } else {
            false
        }
    }

    /// Échange une carte de self avec une carte d'un autre joueur. Retourne true si succès.
    pub fn exchange_card(&mut self, other: &mut Player, my_card_id: crate::server::game::card::CardId, their_card_id: crate::server::game::card::CardId) -> bool {
        let my_pos = self.hand_cards.iter().position(|c| c.get_id() == my_card_id);
        let their_pos = other.hand_cards.iter().position(|c| c.get_id() == their_card_id);
        if let (Some(my_pos), Some(their_pos)) = (my_pos, their_pos) {
            let my_card = self.hand_cards.remove(my_pos);
            let their_card = other.hand_cards.remove(their_pos);
            self.hand_cards.push(their_card);
            other.hand_cards.push(my_card);
            true
        } else {
            false
        }
    }

    /// Retire et retourne une carte aléatoire de la main du joueur, ou None si la main est vide.
    pub fn remove_random_card(&mut self) -> Option<Box<dyn Card>> {
        if self.hand_cards.is_empty() {
            return None;
        }
        let idx = rand::thread_rng().gen_range(0..self.hand_cards.len());
        let card_id = self.hand_cards[idx].get_id();
        println!("[VOL] Joueur {} perd la carte {} (main avant: {})", self.name, card_id, self.hand_cards.len());
        let card = self.hand_cards.remove(idx);
        println!("[VOL] Main après: {}", self.hand_cards.len());
        Some(card)
    }
}
