
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Air,
    Earth,
    Water,
}

//enum natures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nature {
    Spell,
    Weapon,
    Food,
}

//enum étoiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stars {
    One,
    Two,
    Three,
    Four,
    Five,
}

//gestion des id propre à chaque carte
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub nature: Nature,
    pub effect: String,
    pub attack: i32,
    pub heal: i32,
    pub draw: i32,
    pub dice: bool,
}


//constructeur de la carte
impl Card {
    pub fn new(
        name: String,
        element: Element,
        stars: Stars,
        nature: Nature,
        effect: String,
        attack: i32,
        heal: i32,
        draw: i32,
        dice: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            element,
            stars,
            nature,
            effect,
            attack,
            heal,
            draw,
            dice,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_card() {
        let boule_de_feu = Card::new(
            String::from("Boule de feu"),
            Element::Fire,
            Stars::Two,
            Nature::Spell,
            String::from("Inflige 4 points de dégâts."),
            4,
            0,
            0,
            false,
        );

        println!("{:?}", boule_de_feu);
    }
}
