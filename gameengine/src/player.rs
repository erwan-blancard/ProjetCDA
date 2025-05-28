use uuid::Uuid;
use crate::{card::{Card, EffectId, Element}, game::Game, play_info::{ActionTarget, ActionType, PlayInfo}};

const PLAYER_MAX_HEALTH: i32 = 100;

pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub health: i32,
    pub hand_cards: Vec<Box<dyn Card>>,
    pub discard_cards: Vec<Box<dyn Card>>,
    pub casted_cards: Vec<Box<dyn Card>>,
}

impl Player {
    /// Crée un joueur avec un nom donné et des valeurs par défaut
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            health: PLAYER_MAX_HEALTH,
            hand_cards: Vec::new(),
            discard_cards: Vec::new(),
            casted_cards: Vec::new(),
        }
    }

    // pub fn play_card(&self, card: &impl Card, targets: &mut Vec<Player>, game: &mut Game) -> Result<PlayInfo, String> {
    //     card.play(self, targets, game)
    // }

    pub fn damage(&mut self, amount: u32, element: Element, effect: EffectId, game: &Game) -> ActionTarget {
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

    pub fn heal(&mut self, amount: u32, effect: EffectId, game: &Game) -> ActionTarget {
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
