use std::fmt::{self, Debug, Display};

use crate::{game::Game, play_info::{PlayAction, PlayInfo}};


// TODO
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

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

use serde::Deserialize;
//gestion des id propre à chaque carte
use uuid::Uuid;

use crate::player::Player;

#[derive(Debug, Clone)]
pub struct BasicCard {
    pub id: Uuid,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
    pub attack: u32,
    pub heal: u32,
    pub draw: u32,
    pub dice: bool,
}

impl Card for BasicCard {
    fn get_name(&self) -> String { String::from(&self.name) }
    fn get_attack(&self) -> u32 { self.attack }
    fn get_heal(&self) -> u32 { self.heal }
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


//constructeur de la carte
impl BasicCard {
    fn new(
        name: String,
        element: Element,
        stars: Stars,
        kind: Kind,
        desc: String,
        attack: u32,
        heal: u32,
        draw: u32,
        dice: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            element,
            stars,
            kind,
            desc,
            attack,
            heal,
            draw,
            dice,
        }
    }
}


pub trait Card: Sync + Debug +CardClone {
    // basic can_play impl
    fn can_play(&self, player: &Player, targets: &Vec<Player>, game: &Game) -> Result<(), String> {
        self.validate_targets(player, targets, game)
    }

    // basic play impl
    fn play(&self, player: &Player, targets: &mut Vec<Player>, game: &mut Game) -> Result<PlayInfo, String> {
        match self.can_play(player, &targets, game) {
            Ok(_) => {
                let mut info: PlayInfo = PlayInfo::new();
                let mut play_action: PlayAction = PlayAction::new();
                
                match self.get_kind() {
                    Kind::Weapon => {
                        let action_target = targets[0].damage(self.get_attack(), self.get_element(), self.get_damage_effect(), game);
                        play_action.targets.insert(0, action_target);
                    },
                    Kind::Spell => {
                        let action_target = targets[0].damage(self.get_attack(), self.get_element(), self.get_damage_effect(), game);
                        play_action.targets.insert(0, action_target);
                    },
                    Kind::Food => {
                        let action_target = targets[0].heal(self.get_heal(), self.get_heal_effect(), game);
                        play_action.targets.insert(0, action_target);
                    }
                }

                info.actions.insert(0, play_action);

                Ok(info)
            }
            Err(msg) => { Err(msg) }
        }
    }

    fn get_name(&self) -> String { String::from("???") }
    fn get_attack(&self) -> u32 { 1 }
    fn get_heal(&self) -> u32 { 0 }
    fn get_description(&self) -> String { String::from("N/A") }
    fn get_kind(&self) -> Kind { Kind::Weapon }
    fn get_element(&self) -> Element { Element::Fire }
    fn get_stars(&self) -> Stars { Stars::One }
    fn get_target_count(&self) -> u32 { 1 }

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
    fn validate_targets(&self, player: &Player, targets: &Vec<Player>, game: &Game) -> Result<(), String> {
        if targets.len() == usize::try_from(self.get_target_count()).unwrap() {
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