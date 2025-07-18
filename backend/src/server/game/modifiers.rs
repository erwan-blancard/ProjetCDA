use std::{fmt::Debug};
use serde::Deserialize;

use super::{cards::card::Element, eval::EvalOp, player::{Player, PlayerId}};


pub trait Modifier: Sync + Send + Debug + ModifierClone {
    /// Return tuple with new value + dice roll (if used) + player id (if used).
    /// Target is not used for heal and draw.
    fn compute(&self, base_value: u32, player: &Player, target: &Player, dice_roll: Option<u8>) -> (u32, u8, PlayerId);
}

// Allow Box<dyn Modifier> clonning

pub trait ModifierClone {
    fn clone_box(&self) -> Box<dyn Modifier>;
}

impl<T> ModifierClone for T
where
    T: 'static + Modifier + Clone,
{
    fn clone_box(&self) -> Box<dyn Modifier> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Modifier> {
    fn clone(&self) -> Box<dyn Modifier> {
        self.clone_box()
    }
}


/// Enum to use to deserialize the different modifiers from cards.json
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ModifierInfo {
    DiceRollModifier(DiceRollModifier),
    HandSizeModifier(HandSizeModifier),
    DiscardSizeModifier(DiscardSizeModifier),
    HandAndDiceModifier(HandAndDiceModifier),
    HandElementsCountModifier(HandElementsCountModifier),
}

impl ModifierInfo {
    pub fn into_boxed(self) -> Box<dyn Modifier> {
        match self {
            ModifierInfo::DiceRollModifier(m) => Box::new(m),
            ModifierInfo::HandSizeModifier(m) => Box::new(m),
            ModifierInfo::DiscardSizeModifier(m) => Box::new(m),
            ModifierInfo::HandAndDiceModifier(m) => Box::new(m),
            ModifierInfo::HandElementsCountModifier(m) => Box::new(m),
        }
    }
}


fn default_cap() -> u32 { u32::MAX }


#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub struct DiceRollModifier {
    pub dice_op: EvalOp,
    /// maximum value
    #[serde(default = "default_cap")]
    pub cap: u32,
    /// if true, returned id is from target (defends with dice), else id is from player (attacks with dice)
    #[serde(default)]
    pub target_throws_dice: bool,
}

impl Modifier for DiceRollModifier {
    fn compute(&self, base_value: u32, player: &Player, target: &Player, dice_roll: Option<u8>) -> (u32, u8, PlayerId) {
        let dice_roll: u8 = dice_roll.unwrap_or_else(|| rand::random_range(0..6) + 1);
        let mut result: u32 = self.dice_op.eval(base_value, dice_roll as u32);
        // cap result
        if result > self.cap { result = self.cap; }

        println!("DiceRollModifier: base_value={}, dice_roll={}, result={}", base_value, dice_roll, result);

        (result, dice_roll, if self.target_throws_dice { target.id } else { player.id })
    }
}


#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub struct HandSizeModifier {
    pub hand_size_op: EvalOp,
    /// maximum value
    #[serde(default = "default_cap")]
    pub cap: u32,
    /// if true, the modifier does its calculation on the target's hand size, else on the player's hand size
    #[serde(default)]
    pub from_target: bool,
}

impl Modifier for HandSizeModifier {
    fn compute(&self, base_value: u32, player: &Player, target: &Player, _dice_roll: Option<u8>) -> (u32, u8, PlayerId) {
        let hand_size = if self.from_target { target.hand_cards.len() } else { player.hand_cards.len() } as u32;
        let mut result: u32 = self.hand_size_op.eval(base_value, hand_size);
        // cap result
        if result > self.cap { result = self.cap; }

        println!("HandSizeModifier: base_value={}, hand_size={}, result={}", base_value, hand_size, result);

        (result, 0, -1)
    }
}


#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub struct DiscardSizeModifier {
    pub discard_size_op: EvalOp,
    /// maximum value
    #[serde(default = "default_cap")]
    pub cap: u32,
    /// if true, the modifier does its calculation on the target's discard size, else on the player's discard size
    #[serde(default)]
    pub from_target: bool,
}

impl Modifier for DiscardSizeModifier {
    fn compute(&self, base_value: u32, player: &Player, target: &Player, _dice_roll: Option<u8>) -> (u32, u8, PlayerId) {
        let discard_size = if self.from_target { target.discard_cards.len() } else { player.discard_cards.len() } as u32;
        let mut result: u32 = self.discard_size_op.eval(base_value, discard_size);
        // cap result
        if result > self.cap { result = self.cap; }

        println!("DiscardSizeModifier: base_value={}, discard_size={}, result={}", base_value, discard_size, result);

        (result, 0, -1)
    }
}


/// Result value is based on the hand size times the dice roll (base_value is ignored)
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub struct HandAndDiceModifier {
    pub op: EvalOp,
    #[serde(default = "default_cap")]
    pub cap: u32,
    /// if true, returned id is from target (defends with dice), else id is from player (attacks with dice)
    #[serde(default)]
    pub target_throws_dice: bool,
    /// if true, the modifier does its calculation on the target's hand size, else on the player's hand size
    #[serde(default)]
    pub hand_from_target: bool,
}

impl Modifier for HandAndDiceModifier {
    fn compute(&self, _base_value: u32, player: &Player, target: &Player, dice_roll: Option<u8>) -> (u32, u8, PlayerId) {
        let dice_roll: u8 = dice_roll.unwrap_or_else(|| rand::random_range(0..6) + 1);
        let hand_size = if self.hand_from_target { target.hand_cards.len() } else { player.hand_cards.len() } as u32;
        let mut result: u32 = self.op.eval(hand_size, dice_roll as u32);
        // cap result
        if result > self.cap { result = self.cap; }

        println!("HandAndDiceModifier: hand_size={}, dice_roll={}, result={}", hand_size, dice_roll, result);

        (result, dice_roll, if self.target_throws_dice { target.id } else { player.id })
    }
}


/// Modifier tied to the number of cards matching a specific element in the hand of the player or its target
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub struct HandElementsCountModifier {
    pub element: Element,
    pub op: EvalOp,
    /// maximum value
    #[serde(default = "default_cap")]
    pub cap: u32,
    /// if true, the modifier does its calculation on the target's hand, else on the player's hand
    #[serde(default)]
    pub from_target: bool,
}

impl Modifier for HandElementsCountModifier {
    fn compute(&self, base_value: u32, player: &Player, target: &Player, _dice_roll: Option<u8>) -> (u32, u8, PlayerId) {
        let hand = if self.from_target { &target.hand_cards } else { &player.hand_cards };
        let count = hand.iter().filter(|c| c.get_element() == self.element).count() as u32;
        let mut result: u32 = self.op.eval(base_value, count);
        // cap result
        if result > self.cap { result = self.cap; }

        println!("HandElementsCountModifier: base_value={}, count={}, result={}", base_value, count, result);

        (result, 0, -1)
    }
}
