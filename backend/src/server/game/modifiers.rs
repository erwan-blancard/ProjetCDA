use crate::server::game::player::Player;

pub trait Modifier: Send + Sync {
    fn compute(&self, base: u32, player: &mut Player, target: &mut Player, dice_roll: Option<u8>) -> (u32, u8, i32);
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ModifierInfo {
    DiceRollModifier { dice_op: String },
    HandSizeModifier { hand_size_op: String, from_target: Option<bool> },
    HandAndDiceModifier { op: String, hand_from_target: Option<bool> },
    DiscardSizeModifier { discard_size_op: String, from_target: Option<bool> },
}

impl ModifierInfo {
    pub fn into_boxed(self) -> Box<dyn Modifier> {
        // Ici, tu dois compléter avec la logique de création de chaque type de modificateur
        // Pour l'instant, on retourne un modificateur bidon
        Box::new(NoopModifier {})
    }
}

struct NoopModifier;
impl Modifier for NoopModifier {
    fn compute(&self, base: u32, _player: &mut Player, _target: &mut Player, _dice_roll: Option<u8>) -> (u32, u8, i32) {
        (base, 0, -1)
    }
} 