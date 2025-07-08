use serde_derive::{Deserialize, Serialize};

use crate::server::game::player::PlayerId;


/// JSON structures for client messages
#[derive(Serialize, Deserialize, Debug)]
// Tells serde to try to deserialyze the user's JSON action to any of the following structures.
// The JSON must contain the key "type" with a string matching the enum variant's name.
#[serde(tag = "type")]
pub enum UserAction {
    /// The user plays a card on targetted opponents
    /// Dice rolls are handled by the server (no actual dice roll, the client only sees the result of the roll)
    PlayCard {card_index: usize, targets: Vec<PlayerId>},
    /// The user wants to draw a card
    DrawCard {},
    SendChatMessage {message: String,},

}
