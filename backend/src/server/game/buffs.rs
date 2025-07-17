use std::{fmt::Debug};
use serde::{Deserialize, Serialize};

use crate::server::game::{cards::card::{Card, Element, Kind, Stars}, eval::EvalOp};


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum BuffType {
    /// Applicable for the list of elements, kinds and stars.
    /// If no elements, kinds or stars defined, it behaves as if all variants are valid.
    Attack { value: u32, op: EvalOp, elements: Vec<Element>, kinds: Vec<Kind>, stars: Vec<Stars> },
    /// Target all players
    TargetAll,
    /// Play all cards in one turn matching the element
    /// If no elements, kinds or stars defined, it behaves as if all variants are valid.
    PlayAllCards { elements: Vec<Element>, kinds: Vec<Kind>, stars: Vec<Stars> },
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum BuffLifeTime {
    /// Buff is considered used at the end of the turn, regardless if it was used or not.
    UntilNextTurnEnd,
    /// Buff is considered used at the end of the turn if it was used by a card.
    // UntilTurnEndIfUsed,
    /// Buff is considered used when used by a card. It does not apply for the next actions in the turn.
    UntilUsed,
}


/// Trait for buffs that can be applied to cards.
/// Buffs are granted when a card is played.
pub trait Buff: Sync + Send + Debug + BuffClone {
    fn get_type(&self) -> BuffType;
    fn get_lifetime(&self) -> BuffLifeTime { BuffLifeTime::UntilNextTurnEnd }

    // can't use Box<dyn Card> because it can't be called by play() for default impl in Card
    // use element, kind, stars directly instead
    fn is_applicable(&self, card_element: Element, card_kind: Kind, card_stars: Stars) -> bool {
        match self.get_type() {
            BuffType::Attack { value: _, op: _, elements, kinds, stars } => {
                // no elements, kinds or stars defined -> ok (all)
                (elements.len() == 0 || elements.iter().any(|&e| e == card_element))
                && (kinds.len() == 0 || kinds.iter().any(|&k| k == card_kind))
                && (stars.len() == 0 || stars.iter().any(|&s| s == card_stars))
            }
            BuffType::PlayAllCards { elements, kinds, stars } => {
                // no elements, kinds or stars defined -> ok (all)
                (elements.len() == 0 || elements.iter().any(|&e| e == card_element))
                && (kinds.len() == 0 || kinds.iter().any(|&k| k == card_kind))
                && (stars.len() == 0 || stars.iter().any(|&s| s == card_stars))
            }
            _ => { true }
        }
    }

    fn compute(&self, base_value: u32) -> u32 {
        match self.get_type() {
            BuffType::Attack { value, op, elements: _, kinds: _, stars: _ } => {
                op.eval(base_value, value)
            }
            _ => { base_value }
        }
    }

    /// Convert the Buff to a BuffVariant to be able to serialize it with serde (not implemented for Box<dyn Buff>)
    fn as_variant(&self) -> BuffVariant;
}

// Allow Box<dyn Buff> clonning

pub trait BuffClone {
    fn clone_box(&self) -> Box<dyn Buff>;
}

impl<T> BuffClone for T
where
    T: 'static + Buff + Clone,
{
    fn clone_box(&self) -> Box<dyn Buff> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Buff> {
    fn clone(&self) -> Box<dyn Buff> {
        self.clone_box()
    }
}


/// Enum to use to deserialize the different buffs from cards.json
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum BuffVariant {
    AttackBuff(AttackBuff),
    TargetAllBuff(TargetAllBuff),
    PlayAllCardsBuff(PlayAllCardsBuff),
}

impl BuffVariant {
    pub fn into_boxed(self) -> Box<dyn Buff> {
        match self {
            BuffVariant::AttackBuff(b) => Box::new(b),
            BuffVariant::TargetAllBuff(b) => Box::new(b),
            BuffVariant::PlayAllCardsBuff(b) => Box::new(b),
        }
    }
}


fn default_attack_op() -> EvalOp { EvalOp::Add }
fn default_lifetime() -> BuffLifeTime { BuffLifeTime::UntilNextTurnEnd }


#[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(tag = "type")]
pub struct AttackBuff {
    pub value: u32,
    #[serde(default = "default_attack_op")]
    pub op: EvalOp,
    #[serde(default)]
    pub elements: Vec<Element>,
    #[serde(default)]
    pub kinds: Vec<Kind>,
    #[serde(default)]
    pub stars: Vec<Stars>,
    #[serde(default = "default_lifetime")]
    pub lifetime: BuffLifeTime,
}

impl Buff for AttackBuff {
    fn get_lifetime(&self) -> BuffLifeTime { self.lifetime.clone() }
    fn get_type(&self) -> BuffType {
        BuffType::Attack { value: self.value, op: self.op.clone(), elements: self.elements.clone(), kinds: self.kinds.clone(), stars: self.stars.clone() }
    }

    fn as_variant(&self) -> BuffVariant {
        BuffVariant::AttackBuff(self.clone())
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(tag = "type")]
// pub struct TargetAllBuff {}
pub struct TargetAllBuff;

impl Buff for TargetAllBuff {
    fn get_type(&self) -> BuffType { BuffType::TargetAll }

    fn as_variant(&self) -> BuffVariant {
        BuffVariant::TargetAllBuff(self.clone())
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(tag = "type")]
pub struct PlayAllCardsBuff {
    #[serde(default)]
    pub elements: Vec<Element>,
    #[serde(default)]
    pub kinds: Vec<Kind>,
    #[serde(default)]
    pub stars: Vec<Stars>,
}

impl Buff for PlayAllCardsBuff {
    fn get_type(&self) -> BuffType { BuffType::PlayAllCards { elements: self.elements.clone(), kinds: self.kinds.clone(), stars: self.stars.clone() } }

    fn as_variant(&self) -> BuffVariant {
        BuffVariant::PlayAllCardsBuff(self.clone())
    }
}
