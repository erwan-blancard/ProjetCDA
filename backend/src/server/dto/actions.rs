use serde_derive::{Deserialize, Serialize};

use crate::server::game::player::PlayerId;

/// Liste des actions possibles à exécuter côté serveur
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Action {
    Damage { to: PlayerId, amount: i32 },
    Heal { to: PlayerId, amount: i32 },
    Draw { player: PlayerId, count: u8 },
}

/// Structure des messages que le client peut envoyer au serveur
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum UserAction {
    /// Le joueur joue une carte, et demande que le serveur exécute une série d’actions
    PlayCard {
        card_id: u32,
        actions: Vec<Action>,
    },

    /// Le joueur tire une carte (volontairement)
    DrawCard {},

    /// Le joueur envoie un message texte aux autres
    SendChatMessage {
        message: String,
    },
}
