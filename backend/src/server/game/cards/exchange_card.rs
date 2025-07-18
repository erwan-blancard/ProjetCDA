use std::collections::HashSet;

use super::card::{Card, CardId, Element, Kind, Stars, TargetType};
use super::super::modifiers::Modifier;
use super::super::buffs::{Buff, BuffType};
use super::super::game::{Game, MAX_PLAYERS};
use super::super::player::Player;
use super::super::play_info::{PlayAction, PlayInfo, ActionTarget, ActionType};

use crate::server::game::cards::card::check_apply_attack_buffs;
use crate::utils::clamp::clamp;
use serde::Deserialize;
use crate::server::game::cards::card::BasicCard;


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplexEffect {
    Steal,
    Give,
    Exchange,
}

#[derive(Debug, Clone)]
pub struct ComplexEffectCard {
    pub base: BasicCard,
    pub complex_effects: Vec<ComplexEffect>,
}

impl Card for ComplexEffectCard {
    fn get_id(&self) -> CardId { self.base.get_id() }
    fn get_name(&self) -> String { self.base.get_name() }
    fn get_attack(&self) -> u32 { self.base.get_attack() }
    fn get_attack_modifier(&self) -> Option<Box<dyn Modifier>> { self.base.get_attack_modifier() }
    fn get_heal(&self) -> u32 { self.base.get_heal() }
    fn get_heal_modifier(&self) -> Option<Box<dyn Modifier>> { self.base.get_heal_modifier() }
    fn get_draw(&self) -> u32 { self.base.get_draw() }
    fn get_draw_modifier(&self) -> Option<Box<dyn Modifier>> { self.base.get_draw_modifier() }
    fn get_description(&self) -> String { self.base.get_description() }
    fn get_kind(&self) -> Kind { self.base.get_kind() }
    fn get_element(&self) -> Element { self.base.get_element() }
    fn get_stars(&self) -> Stars { self.base.get_stars() }
    fn get_target_type(&self) -> TargetType { self.base.get_target_type() }

    fn play(&self, player_index: usize, target_indices: Vec<usize>, game: &mut Game) -> Result<(PlayInfo, HashSet<usize>), String> {
        let (mut info, mut buffs_used) = self.base.play(player_index, target_indices.clone(), game)?;
        for effect in &self.complex_effects {
            match effect {
                ComplexEffect::Steal => {
                    let target_index = target_indices[0];
                    let (player, target) = if player_index < target_index {
                        let (left, right) = game.players.split_at_mut(target_index);
                        (&mut left[player_index], &mut right[0])
                    } else {
                        let (left, right) = game.players.split_at_mut(player_index);
                        (&mut right[0], &mut left[target_index])
                    };
                    if let Some(stolen_card) = target.remove_random_card() {
                        let stolen_card_id = stolen_card.get_id();
                        println!("[VOL] {} vole la carte {} à {}", player.name, stolen_card_id, target.name);
                        player.hand_cards.push(stolen_card);
                        let mut steal_action = PlayAction::new();
                        steal_action.targets.push(ActionTarget {
                            player_id: player.id,
                            action: ActionType::Steal { cards: vec![stolen_card_id] },
                            effect: "steal".to_string(),
                        });
                        info.actions.push(steal_action);
                    }
                }
                _ => {}
            }
        }
        Ok((info, buffs_used))
    }
}