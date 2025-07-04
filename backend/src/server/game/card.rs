use std::fmt::{self, Debug, Display};
use serde::Deserialize;

use crate::server::game::{modifiers::Modifier, play_info::{ActionTarget, ActionType}, player::PlayerId};

use super::{game::Game, play_info::{PlayAction, PlayInfo}};
use super::player::Player;


// TODO define effects
pub type EffectId = String;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Element {
    Fire,
    Air,
    Earth,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Kind {
    Spell,
    Weapon,
    Food,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Stars {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Display for Stars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub type CardId = i32;

#[derive(Debug, Clone)]
pub struct BasicCard {
    pub id: CardId,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
    pub attack: u32,
    pub attack_modifier: Option<Box<dyn Modifier>>,
    pub heal: u32,
    pub heal_modifier: Option<Box<dyn Modifier>>,
    pub draw: u32,
    pub draw_modifier: Option<Box<dyn Modifier>>,
}

impl Card for BasicCard {
    fn get_id(&self) -> CardId { self.id }
    fn get_name(&self) -> String { String::from(&self.name) }
    fn get_attack(&self) -> u32 { self.attack }
    fn get_attack_modifier(&self) -> Option<Box<dyn Modifier>> { self.attack_modifier.clone() }
    fn get_heal(&self) -> u32 { self.heal }
    fn get_heal_modifier(&self) -> Option<Box<dyn Modifier>> { self.heal_modifier.clone() }
    fn get_draw(&self) -> u32 { self.draw }
    fn get_draw_modifier(&self) -> Option<Box<dyn Modifier>> { self.draw_modifier.clone() }
    fn get_description(&self) -> String { String::from(&self.desc) }
    fn get_kind(&self) -> Kind { self.kind }
    fn get_element(&self) -> Element { self.element }
    fn get_stars(&self) -> Stars { self.stars }
}

// impl Display for BasicCard {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.get_description())
//     }
// }


// FIXME from CardInfo ?
impl BasicCard {
    fn new(
        id: CardId,
        name: String,
        element: Element,
        stars: Stars,
        kind: Kind,
        desc: String,
        attack: u32,
        heal: u32,
        draw: u32,
    ) -> Self {
        Self {
            id,
            name,
            element,
            stars,
            kind,
            desc,
            attack,
            attack_modifier: None,
            heal,
            heal_modifier: None,
            draw,
            draw_modifier: None,
        }
    }
}


pub trait Card: Sync + Send + Debug + CardClone {

    // basic can_play impl
    fn can_play(&self, player: &Player, targets: &Vec<&Player>) -> Result<(), String> {
        self.validate_targets(player, targets)
    }

    // common play impl
    fn play(&self, player_index: usize, target_indices: Vec<usize>, game: &mut Game) -> Result<PlayInfo, String> {
        let targets = target_indices.iter().map(|i| &game.players[*i]).collect();
        match self.can_play(&game.players[player_index], &targets) {
            Ok(_) => {
                let mut info: PlayInfo = PlayInfo::new();

                // attack targets
                if self.get_attack() > 0 || self.get_attack_modifier().is_some() {
                    for target_index in target_indices {
                        let mut attack_action: PlayAction = PlayAction::new();

                        // use split_at_mut() to prevent warnings about mutable borrows
                        let (player, target) = if player_index < target_index {
                            let (left, right) = game.players.split_at_mut(target_index);
                            (&mut left[player_index], &mut right[0])
                        } else if player_index > target_index {
                            let (left, right) = game.players.split_at_mut(player_index);
                            (&mut right[0], &mut left[target_index])
                        } else {
                            return Err("Target is player !".to_string());
                        };
                        
                        let (amount, dice_roll, player_dice_id) = {
                            if let Some(modifier) = self.get_attack_modifier() {
                                modifier.compute(self.get_attack(), player, target)
                            } else { (self.get_attack(), 0, -1) }
                        };

                        attack_action.dice_roll = dice_roll;
                        attack_action.player_dice_id = player_dice_id;

                        let action_target = target.damage(amount, self.get_element(), self.get_damage_effect());
                        attack_action.targets.push(action_target);
                        info.actions.push(attack_action);
                    }
                }

                let player = &mut game.players[player_index];

                if self.get_heal() > 0 || self.get_heal_modifier().is_some() {
                    let mut heal_action: PlayAction = PlayAction::new();

                    let (amount, dice_roll, player_dice_id) = {
                        if let Some(modifier) = self.get_heal_modifier() {
                            modifier.compute(self.get_heal(), player, player)
                        } else { (self.get_heal(), 0, -1) }
                    };

                    heal_action.dice_roll = dice_roll;
                    heal_action.player_dice_id = player_dice_id;

                    let action_target = player.heal(amount, self.get_heal_effect());
                    heal_action.targets.push(action_target);
                    info.actions.push(heal_action);
                }

                if self.get_draw() > 0 || self.get_draw_modifier().is_some() {
                    // FIXME handle discard cards collection later, for now we can't draw more cards than there is in pile

                    let (amount, dice_roll, player_dice_id) = {
                        if let Some(modifier) = self.get_draw_modifier() {
                            modifier.compute(self.get_draw(), player, player)
                        } else { (self.get_draw(), 0, -1) }
                    };

                    let drawn_cards = Game::give_from_pile(&mut game.pile, player, amount as usize);
                    if drawn_cards.len() > 0 {
                        let mut draw_action = PlayAction::new();
                        draw_action.dice_roll = dice_roll;
                        draw_action.player_dice_id = player_dice_id;
                        draw_action.targets.push(ActionTarget {
                            player_id: player.id,
                            action: ActionType::Draw { cards: drawn_cards },    // FIXME set to -1 when sending to clients that aren't the current player
                            effect: String::new()
                        });
                        info.actions.push(draw_action);
                    }
                }

                Ok(info)
            }
            Err(msg) => { Err(msg) }
        }
    }

    fn get_id(&self) -> CardId;
    fn get_name(&self) -> String { String::from("???") }
    fn get_attack(&self) -> u32 { 1 }
    fn get_attack_modifier(&self) -> Option<Box<dyn Modifier>> { None }
    fn get_heal(&self) -> u32 { 0 }
    fn get_heal_modifier(&self) -> Option<Box<dyn Modifier>> { None }
    fn get_draw(&self) -> u32 { 0 }
    fn get_draw_modifier(&self) -> Option<Box<dyn Modifier>> { None }
    fn get_description(&self) -> String { String::from("N/A") }
    fn get_kind(&self) -> Kind { Kind::Weapon }
    fn get_element(&self) -> Element { Element::Fire }
    fn get_stars(&self) -> Stars { Stars::One }
    fn get_target_count(&self) -> usize {
        if self.get_heal() > 0 && self.get_attack() == 0 { 0 } else { 1 }   // no targets if only heal
    }

    fn get_damage_effect(&self) -> EffectId {
        match self.get_element() {
            Element::Air => {
                EffectId::from("damage_air_regular")
            },
            Element::Earth => {
                EffectId::from("damage_earth_regular")
            },
            Element::Fire => {
                EffectId::from("damage_fire_regular")
            }
            Element::Water => {
                EffectId::from("damage_water_regular")
            }
        }
    }

    fn get_heal_effect(&self) -> EffectId { EffectId::from("heal_regular") }

    // basic validate_targets impl (only check if target count is equal to targets len)
    fn validate_targets(&self, player: &Player, targets: &Vec<&Player>) -> Result<(), String> {
        println!("Validate targets: check {:?} == {:?}", targets.len(), self.get_target_count());
        if targets.len() == self.get_target_count() {
            Ok(())
        } else {
            Err(String::from("Invalid target count"))
        }
        
    }

}


// Allow Box<dyn Card> clonning

pub trait CardClone {
    fn clone_box(&self) -> Box<dyn Card>;
}

impl<T> CardClone for T
where
    T: 'static + Card + Clone,
{
    fn clone_box(&self) -> Box<dyn Card> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Card> {
    fn clone(&self) -> Box<dyn Card> {
        self.clone_box()
    }
}