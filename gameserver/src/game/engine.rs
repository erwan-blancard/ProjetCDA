const MAX_PLAYERS: usize = 6;
const CARD_COUNT: usize = 150;
const PLAYER_BASE_HEALTH: u32 = 100;

pub type Card = u32;    // TODO


#[derive(Debug)]
pub struct GameEngine {
    // fields related to game
    pile_cards: Vec<Card>,
    player_healths: [u32; MAX_PLAYERS],
    player_decks: [Vec<Card>; MAX_PLAYERS],
    player_discards: [Vec<Card>; MAX_PLAYERS],
}

impl GameEngine {
    pub fn new() -> GameEngine {
        Self {
            pile_cards: Vec::new(),
            player_healths: [PLAYER_BASE_HEALTH; MAX_PLAYERS],
            player_decks: Default::default(),
            player_discards: Default::default(),

        }
    }
}
