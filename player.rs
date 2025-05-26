use uuid::Uuid;
use crate::card::Card; // N'oublie pas d'importer la struct Card si elle est définie ailleurs

#[derive(Debug, Clone)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub life: i32,
    pub current_hand_card: Vec<Card>,
    pub current_discard_card: Vec<Card>,
    pub attack_boost: i32,
    pub effect: String,
    pub level: i32,
    pub order: i32,
}

impl Player {
    /// Crée un joueur avec un nom donné et des valeurs par défaut
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            life: 100,
            order: 0,
            current_hand_card: Vec::new(),
            current_discard_card: Vec::new(),
            attack_boost: 0,
            effect: String::new(),
            level: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;

    #[test]
    fn test_create_player() {
        let player = Player::new("Player1".to_string());

        assert_eq!(player.name, "Player1");
        assert_eq!(player.life, 100);
        assert_eq!(player.order, 0);
        assert_eq!(player.level, 1);
        assert_eq!(player.current_hand_card.len(), 0);
        assert_eq!(player.current_discard_card.len(), 0);

        println!("{:?}", player);
    }
}
