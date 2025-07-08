use std::fmt::{self, Debug, Display};
use serde::Deserialize;

use crate::utils::clamp::clamp;

use super::game::{Game, MAX_PLAYERS};
use super::play_info::{PlayAction, PlayInfo, ActionTarget, ActionType};
use super::modifiers::Modifier;
use super::player::Player;


// TODO define effects
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

impl Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
pub enum TargetType {
    #[default]
    Single,     // no targets if only heal
    Multiple { max: usize },
    All,
}


pub type CardId = i32;


pub trait Card: Sync + Send + Debug + CardClone {

    // common play impl
    fn play(&self, player_index: usize, target_indices: Vec<usize>, game: &mut Game) -> Result<PlayInfo, String> {
        let targets = target_indices.iter().map(|i| &game.players[*i]).collect();
        match self.validate_targets(&targets) {
            Ok(_) => {
                let mut info: PlayInfo = PlayInfo::new();
                
                let target_indices = {
                    if self.get_target_type() == TargetType::All {
                        game.players.iter().enumerate().filter(|(i, _)| *i != player_index).map(|(i, _)| i).collect()
                    } else { target_indices }
                };

                let dice_roll = rand::random_range(0..6) + 1;   // dice roll value to give to modifiers
                let mut dice_roll_used = false;

                self.handle_attack(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used)?;
                self.handle_heal(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used)?;
                self.handle_draw(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used)?;

                Ok(info)
            }
            Err(msg) => { Err(msg) }
        }
    }

    // basic attack impl
    fn handle_attack(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool) -> Result<(), String> {
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

                let action_target = target.damage(amount, self.get_element(), self.get_damage_effect());
                attack_action.targets.push(action_target);
                info.actions.push(attack_action);
            }
        }

        Ok(())
    }

    // basic heal impl, heal current player
    fn handle_heal(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, _target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool) -> Result<(), String> {
        let player = &mut game.players[player_index];

        if self.get_heal() > 0 || self.get_heal_modifier().is_some() {
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

    // basic draw impl, draw cards for current player
    fn handle_draw(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, _target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool) -> Result<(), String> {
        let player = &mut game.players[player_index];

        if self.get_draw() > 0 || self.get_draw_modifier().is_some() {
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

    fn get_id(&self) -> CardId;
    fn get_name(&self) -> String { String::from("???") }
    fn get_attack(&self) -> u32 { 1 }
    fn get_attack_modifier(&self) -> Option<Box<dyn Modifier>> { None }
    fn get_heal(&self) -> u32 { 0 }
    fn get_heal_modifier(&self) -> Option<Box<dyn Modifier>> { None }
    fn get_draw(&self) -> u32 { 0 }
    fn get_draw_modifier(&self) -> Option<Box<dyn Modifier>> { None }
    fn get_description(&self) -> String { String::from("N/A") }
    fn get_kind(&self) -> Kind { Kind::Weapon }
    fn get_element(&self) -> Element { Element::Fire }
    fn get_stars(&self) -> Stars { Stars::One }
    fn get_target_type(&self) -> TargetType { TargetType::Single }

    fn get_damage_effect(&self) -> EffectId {
        match self.get_element() {
            Element::Air => { EffectId::from("damage_air_regular") },
            Element::Earth => { EffectId::from("damage_earth_regular") },
            Element::Fire => { EffectId::from("damage_fire_regular") }
            Element::Water => { EffectId::from("damage_water_regular") }
        }
    }

    fn get_heal_effect(&self) -> EffectId { EffectId::from("heal_regular") }

    // basic validate_targets impl (only check if target count is equal to targets len)
    fn validate_targets(&self, targets: &Vec<&Player>) -> Result<(), String> {
        println!("Validate targets: target type is {:?}", self.get_target_type());

        let expected =  {
            if self.get_heal() > 0 && self.get_attack() == 0 { 0 }   // no targets if only heal
            else {
                clamp(1, MAX_PLAYERS - 1,
                    match self.get_target_type() {
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


/// Basic card variant that uses the play() impl from Card trait
#[derive(Debug, Clone)]
pub struct BasicCard {
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
}

impl Card for BasicCard {
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
}


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

    // Attack targets multiple times
    fn handle_attack(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, _dice_roll: u8, _dice_roll_used: &mut bool) -> Result<(), String> {
        if self.get_attack() > 0 || self.get_attack_modifier().is_some() {
            for &target_index in target_indices {
                let mut attack_action: PlayAction = PlayAction::new();

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

                for attack in self.attacks.iter() {
                    let action_target = target.damage(*attack, self.get_element(), self.get_damage_effect());
                    attack_action.targets.push(action_target);
                }

                info.actions.push(attack_action);
            }
        }
        
        Ok(())
    }
}


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

    fn handle_attack(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool) -> Result<(), String> {
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

            let action_target = player.damage(amount, self.get_element(), self.get_damage_effect());
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

                let action_target = target.damage(amount, self.get_element(), self.get_damage_effect());
                attack_action.targets.push(action_target);
                info.actions.push(attack_action);
            }
        }

        Ok(())
    }

    fn handle_heal(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool) -> Result<(), String> {
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

    fn handle_draw(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool) -> Result<(), String> {        
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
}

impl MultiActionCard {
    fn get_attack_for_action(&self, action_idx: usize) -> u32 { *self.attacks.get(action_idx).unwrap_or(&0) }
    fn get_heal_for_action(&self, action_idx: usize) -> u32 { *self.heals.get(action_idx).unwrap_or(&0) }
    fn get_draw_for_action(&self, action_idx: usize) -> u32 { *self.draws.get(action_idx).unwrap_or(&0) }
    fn get_target_type_for_action(&self, action_idx: usize) -> TargetType { *self.target_types.get(action_idx).unwrap_or(&TargetType::Single) }
    fn get_attack_modifier_for_action(&self, action_idx: usize) -> Option<Box<dyn Modifier>> { self.attack_modifiers.get(action_idx).map_or(None, |m| m.clone()) }
    fn get_heal_modifier_for_action(&self, action_idx: usize) -> Option<Box<dyn Modifier>> { self.heal_modifiers.get(action_idx).map_or(None, |m| m.clone()) }
    fn get_draw_modifier_for_action(&self, action_idx: usize) -> Option<Box<dyn Modifier>> { self.draw_modifiers.get(action_idx).map_or(None, |m| m.clone()) }

    fn validate_targets_for_action(&self, action_idx: usize, targets: &Vec<&Player>) -> Result<(), String> {
        println!("Validate targets: target type is {:?}", self.get_target_type_for_action(action_idx));

        let expected =  {
            if self.get_heal_for_action(action_idx) > 0 && self.get_attack_for_action(action_idx) == 0 { 0 }   // no targets if only heal
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

    fn handle_attack_for_action(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, action_idx: usize) -> Result<(), String> {
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

                let action_target = target.damage(amount, self.get_element(), self.get_damage_effect());
                attack_action.targets.push(action_target);
                info.actions.push(attack_action);
            }
        }

        Ok(())
    }

    fn handle_heal_for_action(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, _target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, action_idx: usize) -> Result<(), String> {
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

    fn handle_draw_for_action(&self, info: &mut PlayInfo, game: &mut Game, player_index: usize, _target_indices: &Vec<usize>, dice_roll: u8, dice_roll_used: &mut bool, action_idx: usize) -> Result<(), String> {
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

    fn play(&self, player_index: usize, target_indices: Vec<usize>, game: &mut Game) -> Result<PlayInfo, String> {
        let mut info: PlayInfo = PlayInfo::new();

        for action_idx in 0..self.actions {
            let target_indices = {
                if self.get_target_type_for_action(action_idx) == TargetType::All {
                    game.players.iter().enumerate().filter(|(i, _)| *i != player_index).map(|(i, _)| i).collect()
                } else { target_indices.clone() }
            };

            let targets = target_indices.iter().map(|i| &game.players[*i]).collect();
            match self.validate_targets_for_action(action_idx, &targets) {
                Ok(_) => {
                    let dice_roll = rand::random_range(0..6) + 1;   // dice roll value to give to modifiers
                    let mut dice_roll_used = false;

                    self.handle_attack_for_action(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used, action_idx)?;
                    self.handle_heal_for_action(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used, action_idx)?;
                    self.handle_draw_for_action(&mut info, game, player_index, &target_indices, dice_roll, &mut dice_roll_used, action_idx)?;
                }
                Err(msg) => { return Err(msg); }
            };

        }

        Ok(info)
    }
}
