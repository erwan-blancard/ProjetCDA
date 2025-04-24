use serde_derive::{Deserialize, Serialize};

// pub const PLAY_CARD_KEY: &str = "play_card";
// pub const DRAW_CARD_KEY: &str = "draw_card";
// pub const SEND_CHAT_MESSAGE_KEY: &str = "chat_message";


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]      // tells serde to try to deserialyze the user's JSON action to any of the following structures
pub enum UserAction {
    Auth {token: String},
    PlayCard {card_id: i32},
    DrawCard {},
    SendChatMessage {message: String,},

}

// JSON body structure for actions

// #[derive(Serialize, Deserialize, Debug)]
// pub struct PlayCardAction {
//     card_id: i32,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct DrawCardAction {
    
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SendChatMessageAction {
//     message: String,
// }
