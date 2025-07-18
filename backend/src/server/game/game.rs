use std::collections::HashSet;
use std::time::Duration;

use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand::rng;

use super::cards::card::{Card, CardId};
use super::database;
use super::player::{Player, PlayerId};
use super::play_info::PlayInfo;
use super::buffs::BuffLifeTime;

use crate::server::dto::responses::{GameStateForPlayer, OpponentState, PlayerProfile};


pub const MAX_PLAYERS: usize = 6;
pub const INITIAL_HAND_AMOUNT: usize = 5;
pub const DRAW_CARD_LIMIT: usize = 5;   // can't draw if player has more than / or this amount of cards
pub const TURN_DURATION: Duration = Duration::from_secs(90);


#[derive(Debug)]
pub enum Order {
    Forward,
    Backward
}

#[derive(Debug)]
pub enum GameState {
    PreGame,
    InGame,
    EndGame { winner_id: PlayerId }
}


#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub player_profiles: Vec<PlayerProfile>,
    pub pile: Vec<Box<dyn Card>>,
    pub current_player_turn: usize,
    pub current_player_turn_end: DateTime<Utc>,
    /// Estimated amount of time that it will take for the web app to show the actions to the user.
    /// Is determined when playing a card, and reset when the turn is advanced.
    pub estimated_turn_end_offset: Duration,
    pub turn_order: Order,
    pub state: GameState
}

impl Game {
    pub fn new(player_profiles: &Vec<PlayerProfile>) -> Self {
        let players = player_profiles.iter()
            .map(|prf| Player::new(prf.id, prf.name.clone()))
            .collect();

        Self {
            players: players,
            player_profiles: player_profiles.clone(),
            pile: database::CARD_DATABASE.clone(),
            current_player_turn: 0,
            current_player_turn_end: Utc::now(),
            estimated_turn_end_offset: Duration::ZERO,
            turn_order: Order::Forward,
            state: GameState::PreGame,
        }
    }

    /// Distribute cards to players
    /// TODO "throw" dice to determine order
    pub fn begin(&mut self) {
        self.shuffle_pile();
        let pile = &mut self.pile;

        for player in self.players.iter_mut() {
            Self::give_from_pile(pile, player, INITIAL_HAND_AMOUNT);
        }

        self.current_player_turn_end = Utc::now() + TURN_DURATION;

        self.state = GameState::InGame;
    }

    pub fn give_from_pile(pile: &mut Vec<Box<dyn Card>>, player: &mut Player, amount: usize) -> Vec<CardId> {
        let stop = if amount <= pile.len() { amount } else { pile.len() };
        let mut cards = Vec::with_capacity(stop);
        for _ in 0..stop {
            let card = pile.remove(0);
            cards.push(card.get_id());
            player.hand_cards.push(card);
        }

        cards
    }

    pub fn shuffle_pile(&mut self) {
        let mut rng = rng();
        self.pile.shuffle(&mut rng);
    }

    pub fn collect_discard_cards(&mut self) {
        for player in self.players.iter_mut() {
            // empty player discard
            self.pile.append(&mut player.discard_cards);
        }
    }

    pub fn current_player_id(&self) -> PlayerId {
        self.players.get(self.current_player_turn).unwrap().id
    }

    pub fn next_player_index(&self) -> usize {
        match self.turn_order {
            Order::Forward => {
                if self.current_player_turn + 1 >= self.players.len() {
                    0
                } else {
                    self.current_player_turn + 1
                }
            },
            Order::Backward => {
                if self.current_player_turn as i32 - 1 < 0 {
                    self.players.len() - 1
                } else {
                    self.current_player_turn - 1
                }
            }
        }
    }

    pub fn advance_turn(&mut self) {
        self.current_player_turn = self.next_player_index();
        self.current_player_turn_end = Utc::now() + TURN_DURATION + self.estimated_turn_end_offset;
        // reset
        self.estimated_turn_end_offset = Duration::ZERO;
    }

    pub fn play_card(&mut self, player_id: PlayerId, card_index: usize, targets: Vec<PlayerId>) -> Result<PlayInfo, String> {
        if self.current_player_id() != player_id {
            return Err("Not player's current turn".to_string());
        }

        let player_index = self.players
            .iter()
            .position(|p| p.id == player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        let len = targets.len();
        println!("Targets: {}", len);

        // get indices of targets sent by client
        // if card target type is All, it will be ignored by the card
        let mut target_indices = Vec::with_capacity(targets.len());
        for id in targets {
            let idx = self.players
                .iter()
                .position(|p| p.id == id && id != player_id)
                .ok_or_else(|| "Invalid target ID".to_string())?;
            println!("Pushing index: {}", idx);
            target_indices.push(idx);
        }

        let card = self.players[player_index].hand_cards.get(card_index)
            .ok_or_else(|| "Card not in hand".to_string())?.clone();

        // play the card and return play info
        match card.play(player_index, target_indices, self) {
            Ok((play_info, buffs_used)) => {
                // remove used buffs
                self.remove_player_buffs_used(player_index, buffs_used);

                let card = self.players[player_index].hand_cards.remove(card_index);
                // grant card buffs to player
                for buff in card.get_buffs() {
                    self.players[player_index].buffs.push(buff);
                }
                // remove card from hand and put it in discard pile
                self.players[player_index].discard_cards.push(card);

                // check if game is over
                let remaining_players: Vec<&Player> = self.players.iter()
                    .filter(|p| p.health > 0)
                    .collect();

                // state change will be checked by server to send game end event with the winner
                if remaining_players.len() == 1 {
                    self.state = GameState::EndGame { winner_id: remaining_players[0].id };
                } else if remaining_players.len() == 0 {
                    self.state = GameState::EndGame { winner_id: self.current_player_id() };
                }

                self.estimated_turn_end_offset += play_info.get_estimated_time();

                Ok(play_info)
            },
            Err(msg) => { Err(msg) }
        }
    }

    // there should always be at least 1 card in pile when called
    pub fn draw_card(&mut self, player_id: PlayerId) -> Result<CardId, String> {
        let player_index = self.players
            .iter()
            .position(|p| p.id == player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        if self.players[player_index].hand_cards.len() >= DRAW_CARD_LIMIT {
            return Err("Player can't draw more cards".to_string());
        }

        // this should not happen
        if self.pile.len() == 0 {
            return Err("Pile is empty".to_string());
        }
        
        Self::give_from_pile(&mut self.pile, &mut self.players[player_index], 1);

        let card_index = self.players[player_index].hand_cards.len() - 1;
        Ok(self.players[player_index].hand_cards[card_index].get_id())
    }

    fn remove_player_buffs_used(&mut self, player_index: usize, buffs_used: HashSet<usize>) {
        let mut buffs_to_remove: Vec<usize> = Vec::new();

        let player = &mut self.players[player_index];
        for (idx, buff) in player.buffs.iter().enumerate() {
            match buff.get_lifetime() {
                BuffLifeTime::UntilNextTurnEnd => {
                    buffs_to_remove.push(idx);
                }
                BuffLifeTime::UntilUsed => {
                    // remove if used
                    if buffs_used.get(&idx).is_some() {
                        buffs_to_remove.push(idx);
                    }
                }
            }
        }

        buffs_to_remove.sort();
        buffs_to_remove.reverse();

        for &idx in buffs_to_remove.iter() {
            player.buffs.remove(idx);
        }
    }

    pub fn status_for_player(&self, player_id: PlayerId) -> Result<GameStateForPlayer, String> {
        let player_index = self.players
            .iter()
            .position(|p| p.id == player_id)
            .ok_or_else(|| "Player not found".to_string())?;

        let player = &self.players[player_index];

        let opp_states = self.players.iter()
            .filter(|player| player.id != player_id)
            .map(|opp| OpponentState {
                player_id: opp.id,
                health: opp.health as u32,
                card_count: opp.hand_cards.len() as u32,
                discard_cards: opp.discard_cards.iter()
                    .map(|card| card.get_id())
                    .collect(),
                buffs: opp.buffs.iter()
                    .map(|b| b.as_variant())
                    .collect()
            })
            .collect();

        Ok(GameStateForPlayer {
            current_player_turn: self.current_player_id(),
            current_player_turn_end: self.current_player_turn_end,
            health: player.health as u32,
            cards: player.hand_cards.iter()
                .map(|card| card.get_id())
                .collect(),
            discard_cards: player.discard_cards.iter()
                .map(|card| card.get_id())
                .collect(),
            buffs: player.buffs.iter()
                .map(|b| b.as_variant())
                .collect(),
            opponents: opp_states,
            cards_in_pile: self.pile.len() as u32
        })
    }
}

