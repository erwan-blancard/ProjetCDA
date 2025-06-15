use rand::seq::SliceRandom;
use rand::rng;

use crate::server::dto::responses::PlayerProfile;

use super::card::{Card};
use super::database;
use super::player::Player;


pub const MAX_PLAYERS: usize = 6;
const INITIAL_HAND_AMOUNT: usize = 5;


#[derive(Debug)]
pub enum Order {
    Forward,
    Backward
}


#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub player_profiles: Vec<PlayerProfile>,
    pub pile: Vec<Box<dyn Card>>,
    pub current_player_turn: usize,
    pub turn_order: Order
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
            turn_order: Order::Forward
        }
    }

    /// Distribute cards to players
    /// TODO "throw" dice to determine order
    pub fn begin(&mut self) {
        let pile = &mut self.pile;

        for player in self.players.iter_mut() {
            Self::give_from_pile(pile, player, INITIAL_HAND_AMOUNT);
        }
    }

    pub fn give_from_pile(pile: &mut Vec<Box<dyn Card>>, player: &mut Player, amount: usize) {
        for _ in 0..amount {
            player.hand_cards.insert(0, pile.remove(0));
        }
    }

    pub fn shuffle_pile(pile: &mut Vec<Box<dyn Card>>) {
        let mut rng = rng();
        pile.shuffle(&mut rng);
    }

    pub fn collect_discard_cards(&mut self) {
        for player in self.players.iter_mut() {
            // empty player discard
            self.pile.append(&mut player.discard_cards);
        }
    }

    pub fn current_player(&mut self) -> &mut Player {
        self.players.get_mut(self.current_player_turn).unwrap()
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
                if self.current_player_turn - 1 < 0 {
                    self.players.len() - 1
                } else {
                    self.current_player_turn - 1
                }
            }
        }
    }
}

