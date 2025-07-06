use std::{fmt::Debug, u32};
use serde::Deserialize;

use crate::server::game::{card::Element, player::{Player, PlayerId}};


pub trait Modifier: Sync + Send + Debug + ModifierClone {
    /// Return tuple with new value + dice roll (if used) + player id (if used).
    /// Target is not used for heal and draw.
    fn compute(&self, base_value: u32, player: &Player, target: &Player) -> (u32, u8, PlayerId);
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
    HandAndDiceMultModifier(HandAndDiceMultModifier),
}

impl ModifierInfo {
    pub fn into_boxed(self) -> Box<dyn Modifier> {
        match self {
            ModifierInfo::DiceRollModifier(m) => Box::new(m),
            ModifierInfo::HandSizeModifier(m) => Box::new(m),
            ModifierInfo::DiscardSizeModifier(m) => Box::new(m),
            ModifierInfo::HandAndDiceMultModifier(m) => Box::new(m),
        }
    }
}


/// enum that represents the calculation to perform for 2 values
#[derive(Debug, Clone, Deserialize)]
pub enum EvalOp { Add, Sub, Mul, Pow }

impl EvalOp {
    pub fn eval<T:
        std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        // needed for pow (ensure that a and b are u32 to be able to call pow)
        + Copy
        + Into<u32>
        + From<u32>
        >(&self, a: T, b: T) -> T {
        use EvalOp::*;

        match *self {
            Add => { a + b }
            Sub => { a - b }
            Mul => { a * b }
            Pow => { 
                let a_u32: u32 = a.into();
                let b_u32: u32 = b.into();
                let result = a_u32.pow(b_u32);
                T::from(result)
            }
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
    fn compute(&self, base_value: u32, player: &Player, target: &Player) -> (u32, u8, PlayerId) {
        let dice_roll: u8 = rand::random_range(0..6) + 1;
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
    fn compute(&self, base_value: u32, player: &Player, target: &Player) -> (u32, u8, PlayerId) {
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
    fn compute(&self, base_value: u32, player: &Player, target: &Player) -> (u32, u8, PlayerId) {
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
pub struct HandAndDiceMultModifier {
    #[serde(default = "default_cap")]
    pub cap: u32,
    /// if true, returned id is from target (defends with dice), else id is from player (attacks with dice)
    #[serde(default)]
    pub target_throws_dice: bool,
    /// if true, the modifier does its calculation on the target's hand size, else on the player's hand size
    #[serde(default)]
    pub hand_from_target: bool,
}

impl Modifier for HandAndDiceMultModifier {
    fn compute(&self, _base_value: u32, player: &Player, target: &Player) -> (u32, u8, PlayerId) {
        let dice_roll: u8 = rand::random_range(0..6) + 1;
        let hand_size = if self.hand_from_target { target.hand_cards.len() } else { player.hand_cards.len() } as u32;
        let mut result: u32 = hand_size * dice_roll as u32;
        // cap result
        if result > self.cap { result = self.cap; }

        println!("HandAndDiceMultModifier: hand_size={}, dice_roll={}, result={}", hand_size, dice_roll, result);

        (result, dice_roll, if self.target_throws_dice { target.id } else { player.id })
    }
}
