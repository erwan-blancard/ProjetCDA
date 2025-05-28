use std::ops::Deref;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card};
use crate::database;
use crate::player::Player;


const INITIAL_HAND_AMOUNT: usize = 5;


pub enum Order {
    Forward,
    Backward
}


pub struct Game {
    players: Vec<Player>,
    pile: Vec<Box<dyn Card>>,
    current_player_turn: usize,
    turn_order: Order
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            players: players,
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
        let mut rng = thread_rng();
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

