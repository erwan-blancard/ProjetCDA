use std::error::Error;

use chrono::Utc;
use diesel::expression::is_aggregate::No;
use rand::seq::SliceRandom;
use rand::rng;

use crate::server::dto::responses::{GameStateForPlayer, OpponentState, PlayerProfile};
use crate::server::game::card::{CardId, Kind};
use crate::server::game::play_info::{PlayAction, PlayInfo};
use crate::server::game::player::PlayerId;

use super::card::{Card};
use super::database;
use super::player::Player;


pub const MAX_PLAYERS: usize = 6;
const INITIAL_HAND_AMOUNT: usize = 5;
const DRAW_CARD_LIMIT: usize = 5;   // can't draw if player has more than / or this amount of cards


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
            turn_order: Order::Forward,
            state: GameState::PreGame,
        }
    }

    /// Distribute cards to players
    /// TODO "throw" dice to determine order
    pub fn begin(&mut self) {
        let pile = &mut self.pile;

        for player in self.players.iter_mut() {
            Self::give_from_pile(pile, player, INITIAL_HAND_AMOUNT);
        }

        self.state = GameState::InGame;
    }

    pub fn give_from_pile(pile: &mut Vec<Box<dyn Card>>, player: &mut Player, amount: usize) {
        for _ in 0..amount {
            player.hand_cards.push(pile.remove(0));
        }
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
        match card.play(player_index, target_indices, &mut self.players) {
            Ok(play_info) => {
                // remove card from hand and put it in discard pile
                let card = self.players[player_index].hand_cards.remove(card_index);
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
                    .collect()
            })
            .collect();

        Ok(GameStateForPlayer {
            current_player_turn: self.current_player_id(),
            current_player_turn_end: Utc::now(),
            health: player.health as u32,
            cards: player.hand_cards.iter()
                .map(|card| card.get_id())
                .collect(),
            discard_cards: player.discard_cards.iter()
                .map(|card| card.get_id())
                .collect(),
            opponents: opp_states,
            cards_in_pile: self.pile.len() as u32
        })
    }
}

