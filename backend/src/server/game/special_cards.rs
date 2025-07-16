use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use serde::Deserialize;

use crate::server::game::card::TargetType;
use crate::utils::clamp::clamp;

use super::game::{Game, MAX_PLAYERS};
use super::play_info::{PlayAction, PlayInfo, ActionTarget, ActionType};
use super::modifiers::Modifier;
use super::player::Player;
use super::card::{Card, CardId, Element, Stars, Kind};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum PlayersRollsDiceCardAction {
    /// All players are affected individually by the card's effects.
    AffectsAllPlayers,
    /// Affects all players with the lowest roll.
    /// The amount is the sum of the dice rolls of each player.
    AffectsMinRollPlayersRollsSum,
}


/// Enum used for PlayersRollsDiceCard's process_dice_action() function.
pub enum PlayersRollsDiceCardActionType { Attack, Heal, Draw, }

impl PlayersRollsDiceCardActionType {
    fn process_action(&self, amount: u32, player_index: usize, card: &dyn Card, game: &mut Game) -> ActionTarget {
        let player = &mut game.players[player_index];

        match self {
            PlayersRollsDiceCardActionType::Attack => {
                player.damage(amount, card.get_element(), card.get_damage_effect())
            }
            PlayersRollsDiceCardActionType::Heal => {
                player.heal(amount, card.get_heal_effect())
            }
            PlayersRollsDiceCardActionType::Draw => {
                let drawn_cards = Game::give_from_pile(&mut game.pile, player, amount as usize);
                ActionTarget {
                    player_id: player.id,
                    action: ActionType::Draw { cards: drawn_cards },    // FIXME set to -1 when sending to clients that aren't the current player
                    effect: String::new()
                }
            }
        }
    }
}


/// Card variant that requires the player and its targets to roll the dice.
/// Each player is affected by the card's effects based on their dice roll.
#[derive(Debug, Clone)]
pub struct PlayersRollsDiceCard {
    pub id: CardId,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
    pub target_type: TargetType,
    pub attack: bool,
    pub heal: bool,
    pub draw: bool,
    pub dice_action: PlayersRollsDiceCardAction,
}

impl PlayersRollsDiceCardAction {

    // dice_rolls contains the dice rolls of the player (0) and its targets (1..n)
    fn process_dice_action(&self, card: &dyn Card, action_type: PlayersRollsDiceCardActionType, info: &mut PlayInfo, game: &mut Game, player_index: usize, target_indices: &Vec<usize>, dice_rolls: &mut Vec<u32>) -> Result<(), String> {
        match self {
            PlayersRollsDiceCardAction::AffectsAllPlayers => {
                let mut action: PlayAction = PlayAction::new();

                for idx in 0..dice_rolls.len() {
                    let action_target = action_type.process_action(dice_rolls[idx],
                        if idx == 0 { player_index } else { target_indices[idx - 1] },
                        card, game);
                    action.targets.push(action_target);
                }

                info.actions.push(action);
            }
            PlayersRollsDiceCardAction::AffectsMinRollPlayersRollsSum => {
                let amount: u32 = dice_rolls.iter().sum();

                // find players who rolled the lowest dice
                let min_dice_roll = *dice_rolls.iter().min().unwrap();
                let min_dice_roll_indexes: Vec<usize> = dice_rolls.iter().enumerate()
                    .filter(|(_, &roll)| roll == min_dice_roll)
                    .map(|(idx, _)| idx)
                    .collect();

                let mut action: PlayAction = PlayAction::new();

                for idx in min_dice_roll_indexes {
                    let action_target = action_type.process_action(amount,
                        if idx == 0 { player_index } else { target_indices[idx - 1] } ,
                        card, game);
                    action.targets.push(action_target);
                }

                info.actions.push(action);
            }
        };

        Ok(())
    }
    
}

impl Card for PlayersRollsDiceCard {
    fn get_id(&self) -> CardId { self.id }
    fn get_name(&self) -> String { String::from(&self.name) }
    fn get_description(&self) -> String { String::from(&self.desc) }
    fn get_kind(&self) -> Kind { self.kind }
    fn get_element(&self) -> Element { self.element }
    fn get_stars(&self) -> Stars { self.stars }
    fn get_target_type(&self) -> TargetType { self.target_type }

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

                let mut dice_rolls: Vec<u32> = Vec::with_capacity(target_indices.len() + 1);
                let dice_roll = rand::random_range(0..6) + 1;   // dice roll value to give to modifiers
                dice_rolls.push(dice_roll as u32);

                // push action
                let mut dice_roll_action: PlayAction = PlayAction::new();
                dice_roll_action.dice_roll = dice_roll;
                dice_roll_action.player_dice_id = game.players[player_index].id;
                info.actions.push(dice_roll_action);


                // generate the other dice rolls
                for &target_index in target_indices.iter() {
                    let mut dice_roll_action: PlayAction = PlayAction::new();
                    
                    let dice_roll = rand::random_range(0..6) + 1;
                    dice_rolls.push(dice_roll as u32);

                    dice_roll_action.dice_roll = dice_roll;
                    dice_roll_action.player_dice_id = game.players[target_index].id;
                    info.actions.push(dice_roll_action);
                }

                if self.attack {
                    self.dice_action.process_dice_action(self, PlayersRollsDiceCardActionType::Attack, &mut info, game, player_index, &target_indices, &mut dice_rolls)?;
                } else if self.heal {
                    self.dice_action.process_dice_action(self, PlayersRollsDiceCardActionType::Heal, &mut info, game, player_index, &target_indices, &mut dice_rolls)?;
                } else if self.draw {
                    self.dice_action.process_dice_action(self, PlayersRollsDiceCardActionType::Draw, &mut info, game, player_index, &target_indices, &mut dice_rolls)?;
                }

                Ok(info)
            }
            Err(msg) => { Err(msg) }
        }
    }
}


/// Card variant for Pearth card
#[derive(Debug, Clone)]
pub struct PearthCard {
    pub id: CardId,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
}

impl Card for PearthCard {
    fn get_id(&self) -> CardId { self.id }
    fn get_name(&self) -> String { String::from(&self.name) }
    fn get_description(&self) -> String { String::from(&self.desc) }
    fn get_kind(&self) -> Kind { self.kind }
    fn get_element(&self) -> Element { self.element }
    fn get_stars(&self) -> Stars { self.stars }
    fn get_target_type(&self) -> TargetType { TargetType::All }

    fn play(&self, player_index: usize, _target_indices: Vec<usize>, game: &mut Game) -> Result<PlayInfo, String> {
        let mut info: PlayInfo = PlayInfo::new();

        let dicards = game.players.iter().map(|player| player.discard_cards.len()).sum::<usize>();

        let mut action: PlayAction = PlayAction::new();
        let action_target = game.players[player_index].heal(dicards as u32, self.get_heal_effect());
        action.targets.push(action_target);

        info.actions.push(action);

        Ok(info)
    }
}
