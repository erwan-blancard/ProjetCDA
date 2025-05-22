use serde_derive::{Deserialize, Serialize};


/// JSON structures for client messages
#[derive(Serialize, Deserialize, Debug)]
// Tells serde to try to deserialyze the user's JSON action to any of the following structures.
// The JSON must contain the key "type" with a string matching the enum variant's name.
#[serde(tag = "type")]
pub enum UserAction {
    Auth {token: String},
    PlayCard {card_id: u32},
    DrawCard {},
    SendChatMessage {message: String,},

}
