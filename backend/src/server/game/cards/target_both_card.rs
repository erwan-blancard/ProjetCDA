use std::collections::HashSet;

use crate::server::game::cards::card::check_apply_attack_buffs;

use super::card::{Card, CardId, Element, Kind, Stars, TargetType};
use super::super::modifiers::Modifier;
use super::super::buffs::Buff;
use super::super::game::Game;
use super::super::play_info::{PlayAction, PlayInfo, ActionTarget, ActionType};


/// Card variant that applies damage, heal and draw to the player and its targets
#[derive(Debug, Clone)]
pub struct TargetBothCard {
    pub id: CardId,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
    pub target_type: TargetType,
    pub attack: u32,
    pub attack_modifier: Option<Box<dyn Modifier>>,
    pub heal: u32,
    pub heal_modifier: Option<Box<dyn Modifier>>,
    pub draw: u32,
    pub draw_modifier: Option<Box<dyn Modifier>>,
    pub buffs: Vec<Box<dyn Buff>>
}

impl Card for TargetBothCard {
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
    fn get_target_type(&self) -> TargetType { self.target_type }

    fn get_buffs(&self) -> Vec<Box<dyn Buff>> { self.buffs.clone() }

    fn handle_attack(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, buffs_used: &mut HashSet<usize>) -> Result<(), String> {
        if self.get_attack() > 0 || self.get_attack_modifier().is_some() {
            let mut attack_self_action: PlayAction = PlayAction::new();

            // attack self
            let player = &mut game.players[player_index];

            let (amount, dice_roll, player_dice_id) = {
                if let Some(modifier) = self.get_attack_modifier() {
                    modifier.compute(self.get_attack(), player, player, Some(dice_roll))
                } else { (self.get_attack(), 0, -1) }
            };

            // show dice anim on client only if this is the first time the dice is used
            if !*dice_roll_used && player_dice_id != -1 {
                attack_self_action.dice_roll = dice_roll;
                attack_self_action.player_dice_id = player_dice_id;
                *dice_roll_used = true;
            }

            // apply attack buffs
            let amount = check_apply_attack_buffs(amount, &player.buffs, self.get_element(), self.get_kind(), self.get_stars(), buffs_used);

            let action_target = player.damage(amount, self.get_damage_effect());
            attack_self_action.targets.push(action_target);
            info.actions.push(attack_self_action);

            // normal impl
            // attack targets
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
                    if let Some(modifier) = self.get_attack_modifier() {
                        modifier.compute(self.get_attack(), player, target, Some(dice_roll))
                    } else { (self.get_attack(), 0, -1) }
                };

                // show dice anim on client only if this is the first time the dice is used
                if !*dice_roll_used && player_dice_id != -1 {
                    attack_action.dice_roll = dice_roll;
                    attack_action.player_dice_id = player_dice_id;
                    *dice_roll_used = true;
                }

                // apply attack buffs
                let amount = check_apply_attack_buffs(amount, &player.buffs, self.get_element(), self.get_kind(), self.get_stars(), buffs_used);

                let action_target = target.damage(amount, self.get_damage_effect());
                attack_action.targets.push(action_target);
                info.actions.push(attack_action);
            }
        }

        Ok(())
    }

    fn handle_heal(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, _buffs_used: &mut HashSet<usize>) -> Result<(), String> {
        if self.get_heal() > 0 || self.get_heal_modifier().is_some() {
            for &target_index in target_indices {
                let mut heal_target_action: PlayAction = PlayAction::new();

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
                    if let Some(modifier) = self.get_heal_modifier() {
                        modifier.compute(self.get_heal(), target, player, Some(dice_roll))
                    } else { (self.get_heal(), 0, -1) }
                };

                // show dice anim on client only if this is the first time the dice is used
                if !*dice_roll_used && player_dice_id != -1 {
                    heal_target_action.dice_roll = dice_roll;
                    heal_target_action.player_dice_id = player_dice_id;
                    *dice_roll_used = true;
                }

                let action_target = target.heal(amount, self.get_heal_effect());
                heal_target_action.targets.push(action_target);
                info.actions.push(heal_target_action);
            }

            // do normal heal for current player
            let player = &mut game.players[player_index];

            let mut heal_action: PlayAction = PlayAction::new();

            let (amount, dice_roll, player_dice_id) = {
                if let Some(modifier) = self.get_heal_modifier() {
                    modifier.compute(self.get_heal(), player, player, Some(dice_roll))
                } else { (self.get_heal(), 0, -1) }
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

    fn handle_draw(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, _buffs_used: &mut HashSet<usize>) -> Result<(), String> {        
        if self.get_draw() > 0 || self.get_draw_modifier().is_some() {
            for &target_index in target_indices {
                // use split_at_mut() to prevent warnings about mutable borrows
                let (_, target) = if player_index < target_index {
                    let (left, right) = game.players.split_at_mut(target_index);
                    (&mut left[player_index], &mut right[0])
                } else if player_index > target_index {
                    let (left, right) = game.players.split_at_mut(player_index);
                    (&mut right[0], &mut left[target_index])
                } else {
                    return Err("Target is player !".to_string());
                };
                
                let (amount, dice_roll, player_dice_id) = {
                    if let Some(modifier) = self.get_draw_modifier() {
                        modifier.compute(self.get_draw(), target, target, Some(dice_roll))
                    } else { (self.get_draw(), 0, -1) }
                };

                let drawn_cards = Game::give_from_pile(&mut game.pile, target, amount as usize);
                if drawn_cards.len() > 0 {
                    let mut draw_target_action = PlayAction::new();
                    
                    // show dice anim on client only if this is the first time the dice is used
                    if !*dice_roll_used && player_dice_id != -1 {
                        draw_target_action.dice_roll = dice_roll;
                        draw_target_action.player_dice_id = player_dice_id;
                        *dice_roll_used = true;
                    }

                    draw_target_action.targets.push(ActionTarget {
                        player_id: target.id,
                        action: ActionType::Draw { cards: drawn_cards },    // FIXME set to -1 when sending to clients that aren't the current player
                        effect: String::new()
                    });
                    info.actions.push(draw_target_action);
                }
            }

            // do normal draw for current player
            let player = &mut game.players[player_index];
            
            // FIXME handle discard cards collection later, for now we can't draw more cards than there is in pile

            let (amount, dice_roll, player_dice_id) = {
                if let Some(modifier) = self.get_draw_modifier() {
                    modifier.compute(self.get_draw(), player, player, Some(dice_roll))
                } else { (self.get_draw(), 0, -1) }
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
