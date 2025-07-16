use std::collections::HashSet;

use crate::server::game::cards::card::check_apply_attack_buffs;

use super::card::{Card, CardId, Element, Kind, Stars};
use super::super::modifiers::Modifier;
use super::super::buffs::Buff;
use super::super::game::Game;
use super::super::play_info::{PlayAction, PlayInfo};


/// Card variant that can attack its target multiple times.
/// This is only for visual effects on the client, showing separate animations for each hit.
/// Attack modifiers are not supported.
#[derive(Debug, Clone)]
pub struct MultiHitCard {
    pub id: CardId,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
    pub attacks: Vec<u32>,
    pub heal: u32,
    pub heal_modifier: Option<Box<dyn Modifier>>,
    pub draw: u32,
    pub draw_modifier: Option<Box<dyn Modifier>>,
    pub buffs: Vec<Box<dyn Buff>>
}

impl Card for MultiHitCard {
    fn get_id(&self) -> CardId { self.id }
    fn get_name(&self) -> String { String::from(&self.name) }
    fn get_heal(&self) -> u32 { self.heal }
    fn get_heal_modifier(&self) -> Option<Box<dyn Modifier>> { self.heal_modifier.clone() }
    fn get_draw(&self) -> u32 { self.draw }
    fn get_draw_modifier(&self) -> Option<Box<dyn Modifier>> { self.draw_modifier.clone() }
    fn get_description(&self) -> String { String::from(&self.desc) }
    fn get_kind(&self) -> Kind { self.kind }
    fn get_element(&self) -> Element { self.element }
    fn get_stars(&self) -> Stars { self.stars }

    fn get_buffs(&self) -> Vec<Box<dyn Buff>> { self.buffs.clone() }

    // Attack targets multiple times
    fn handle_attack(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, _dice_roll: u8, _dice_roll_used: &mut bool, buffs_used: &mut HashSet<usize>) -> Result<(), String> {
        if self.get_attack() > 0 || self.get_attack_modifier().is_some() {
            for &target_index in target_indices {
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

                for attack in self.attacks.iter() {
                    let amount = check_apply_attack_buffs(*attack, &player.buffs, self.get_element(), self.get_kind(), self.get_stars(), buffs_used);
                    let action_target = target.damage(amount, self.get_damage_effect());
                    attack_action.targets.push(action_target);
                }

                info.actions.push(attack_action);
            }
        }
        
        Ok(())
    }
}
