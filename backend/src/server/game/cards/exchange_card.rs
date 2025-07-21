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
                    // Pick a random card index from the victim's hand
                    use rand::Rng;
                    if target_index < game.players.len() && !game.players[target_index].hand_cards.is_empty() {
                        let hand_len = game.players[target_index].hand_cards.len();
                        let card_index = rand::thread_rng().gen_range(0..hand_len);
                        // Remove the card from the victim and add it to the thief
                        let stolen_card = game.players[target_index].hand_cards.remove(card_index);
                        game.players[player_index].hand_cards.push(stolen_card.clone_box());
                        // Log the steal event for debugging
                        println!("[STEAL] Player {} steals a card at index {} from player {}", player_index, card_index, target_index);
                        // Generate a PlayAction/GameEvent for the frontend with the index
                        let mut steal_action = PlayAction::new();
                        steal_action.targets.push(ActionTarget {
                            player_id: player_index as i32, // thief
                            action: ActionType::Steal { cards: vec![card_index as i32] }, // index of the stolen card
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