use std::collections::HashSet;

use super::card::{Card, CardId, Element, Kind, Stars, TargetType};
use super::super::modifiers::Modifier;
use super::super::buffs::{Buff, BuffType};
use super::super::game::{Game, MAX_PLAYERS};
use super::super::player::Player;
use super::super::play_info::{PlayAction, PlayInfo, ActionTarget, ActionType};

use crate::server::game::cards::card::check_apply_attack_buffs;
use crate::utils::clamp::clamp;


/// Card variant that runs the play logic as many times as necessary with different values
#[derive(Debug, Clone)]
pub struct MultiActionCard {
    pub id: CardId,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
    pub actions: usize,
    pub target_types: Vec<TargetType>,
    pub attacks: Vec<u32>,
    pub attack_modifiers: Vec<Option<Box<dyn Modifier>>>,
    pub heals: Vec<u32>,
    pub heal_modifiers: Vec<Option<Box<dyn Modifier>>>,
    pub draws: Vec<u32>,
    pub draw_modifiers: Vec<Option<Box<dyn Modifier>>>,
    pub buffs: Vec<Box<dyn Buff>>
}

impl MultiActionCard {
    fn get_attack_for_action(&self, action_idx: usize) -> u32 { *self.attacks.get(action_idx).unwrap_or(&0) }
    fn get_heal_for_action(&self, action_idx: usize) -> u32 { *self.heals.get(action_idx).unwrap_or(&0) }
    fn get_draw_for_action(&self, action_idx: usize) -> u32 { *self.draws.get(action_idx).unwrap_or(&0) }
    fn get_target_type_for_action(&self, action_idx: usize) -> TargetType { *self.target_types.get(action_idx).unwrap_or(&TargetType::Single) }
    fn get_attack_modifier_for_action(&self, action_idx: usize) -> Option<Box<dyn Modifier>> { self.attack_modifiers.get(action_idx).map_or(None, |m| m.clone()) }
    fn get_heal_modifier_for_action(&self, action_idx: usize) -> Option<Box<dyn Modifier>> { self.heal_modifiers.get(action_idx).map_or(None, |m| m.clone()) }
    fn get_draw_modifier_for_action(&self, action_idx: usize) -> Option<Box<dyn Modifier>> { self.draw_modifiers.get(action_idx).map_or(None, |m| m.clone()) }

    fn get_buffs(&self) -> Vec<Box<dyn Buff>> { self.buffs.clone() }

    fn validate_targets_for_action(&self, action_idx: usize, targets: &Vec<&Player>) -> Result<(), String> {
        println!("Validate targets: target type is {:?}", self.get_target_type_for_action(action_idx));

        let expected =  {
            if (self.get_attack_for_action(action_idx) == 0 && self.get_attack_modifier_for_action(action_idx).is_none())
                && ((self.get_heal_for_action(action_idx) > 0 || self.get_heal_modifier_for_action(action_idx).is_some())
                || (self.get_draw_for_action(action_idx) > 0 || self.get_draw_modifier_for_action(action_idx).is_some())) { 0 }   // no targets if only heal and/or draw
            else {
                clamp(1, MAX_PLAYERS - 1,
                    match self.get_target_type_for_action(action_idx) {
                            TargetType::Single => 1,
                            TargetType::Multiple { max } => clamp(1, max, targets.len()),
                            TargetType::All => { return Ok(()); }   // "targets" is ignored, all players are valid targets except the player itself
                    }
                )
            }
        };

        println!("Validate targets: check {:?} == {:?}", targets.len(), expected);
        if targets.len() == expected {
            Ok(())
        } else {
            Err(String::from("Invalid target count"))
        }
    }

    fn handle_attack_for_action(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, action_idx: usize, buffs_used: &mut HashSet<usize>) -> Result<(), String> {
        if self.get_attack_for_action(action_idx) > 0 || self.get_attack_modifier_for_action(action_idx).is_some() {
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
                
                let (amount, dice_roll, player_dice_id) = {
                    if let Some(modifier) = self.get_attack_modifier_for_action(action_idx) {
                        modifier.compute(self.get_attack_for_action(action_idx), player, target, Some(dice_roll))
                    } else { (self.get_attack_for_action(action_idx), 0, -1) }
                };

                // show dice anim on client only if this is the first time the dice is used
                if !*dice_roll_used && player_dice_id != -1 {
                    attack_action.dice_roll = dice_roll;
                    attack_action.player_dice_id = player_dice_id;
                    *dice_roll_used = true;
                }

                let amount = check_apply_attack_buffs(amount, &player.buffs, self.get_element(), self.get_kind(), self.get_stars(), buffs_used);

                let action_target = target.damage(amount, self.get_damage_effect());
                attack_action.targets.push(action_target);
                info.actions.push(attack_action);
            }
        }

        Ok(())
    }

    fn handle_heal_for_action(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, _target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, action_idx: usize, _buffs_used: &mut HashSet<usize>) -> Result<(), String> {
        let player = &mut game.players[player_index];

        if self.get_heal_for_action(action_idx) > 0 || self.get_heal_modifier_for_action(action_idx).is_some() {
            let mut heal_action: PlayAction = PlayAction::new();

            let (amount, dice_roll, player_dice_id) = {
                if let Some(modifier) = self.get_heal_modifier_for_action(action_idx) {
                    modifier.compute(self.get_heal(), player, player, Some(dice_roll))
                } else { (self.get_heal_for_action(action_idx), 0, -1) }
            };

            // show dice anim on client only if this is the first time the dice is used
            if !*dice_roll_used && player_dice_id != -1 {
                heal_action.dice_roll = dice_roll;
                heal_action.player_dice_id = player_dice_id;
                *dice_roll_used = true;
            }

            let action_target = player.heal(amount, self.get_heal_effect());
            heal_action.targets.push(action_target);
            info.actions.push(heal_action);
        }

        Ok(())
    }

    fn handle_draw_for_action(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, _target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, action_idx: usize, _buffs_used: &mut HashSet<usize>) -> Result<(), String> {
        let player = &mut game.players[player_index];

        if self.get_draw_for_action(action_idx) > 0 || self.get_draw_modifier_for_action(action_idx).is_some() {
            // FIXME handle discard cards collection later, for now we can't draw more cards than there is in pile

            let (amount, dice_roll, player_dice_id) = {
                if let Some(modifier) = self.get_draw_modifier_for_action(action_idx) {
                    modifier.compute(self.get_draw_for_action(action_idx), player, player, Some(dice_roll))
                } else { (self.get_draw_for_action(action_idx), 0, -1) }
            };

            let drawn_cards = Game::give_from_pile(&mut game.pile, player, amount as usize);
            if drawn_cards.len() > 0 {
                let mut draw_action = PlayAction::new();
                
                // show dice anim on client only if this is the first time the dice is used
                if !*dice_roll_used && player_dice_id != -1 {
                    draw_action.dice_roll = dice_roll;
                    draw_action.player_dice_id = player_dice_id;
                    *dice_roll_used = true;
                }

                draw_action.targets.push(ActionTarget {
                    player_id: player.id,
                    action: ActionType::Draw { cards: drawn_cards },    // FIXME set to -1 when sending to clients that aren't the current player
                    effect: String::new()
                });
                info.actions.push(draw_action);
            }
        }

        Ok(())
    }
}

impl Card for MultiActionCard {
    fn get_id(&self) -> CardId { self.id }
    fn get_name(&self) -> String { String::from(&self.name) }
    fn get_description(&self) -> String { String::from(&self.desc) }
    fn get_kind(&self) -> Kind { self.kind }
    fn get_element(&self) -> Element { self.element }
    fn get_stars(&self) -> Stars { self.stars }

    fn play(&self, player_index: usize, target_indices: Vec<usize>, game: &mut Game) -> Result<(PlayInfo, HashSet<usize>), String> {
        let mut info: PlayInfo = PlayInfo::new();
        let mut buffs_used: HashSet<usize> = HashSet::new();

        for action_idx in 0..self.actions {
            let target_indices = {
                if self.get_target_type_for_action(action_idx) == TargetType::All || game.players[player_index].buffs.iter().any(|b| b.get_type() == BuffType::TargetAll) {
                    game.players.iter().enumerate().filter(|(i, _)| *i != player_index).map(|(i, _)| i).collect()
                } else { target_indices.clone() }
            };

            let targets = target_indices.iter().map(|i| &game.players[*i]).collect();
            match self.validate_targets_for_action(action_idx, &targets) {
                Ok(_) => {
                    let dice_roll = rand::random_range(0..6) + 1;   // dice roll value to give to modifiers
                    let mut dice_roll_used = false;

                    self.handle_attack_for_action(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used, action_idx, &mut buffs_used)?;
                    self.handle_heal_for_action(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used, action_idx, &mut buffs_used)?;
                    self.handle_draw_for_action(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used, action_idx, &mut buffs_used)?;
                }
                Err(msg) => { return Err(msg); }
            };

        }

        Ok((info, buffs_used))
    }
}
