use std::{fs::File, io::BufReader, path::Path};

use lazy_static::lazy_static;
use serde::{Deserialize};
use uuid::Uuid;

use crate::card::{BasicCard, Card, Element, Kind, Stars};

lazy_static! {
    pub static ref CARD_DATABASE: Vec<Box<dyn Card>> = {
        create_deck_database()
    };
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum CardVariant {
    BasicCard(BasicCardData),
}

#[derive(Debug, Deserialize)]
struct BasicCardData {
    #[serde(default)]
    attack: u32,
    #[serde(default)]
    heal: u32,
    #[serde(default)]
    draw: u32,
    #[serde(default)]
    dice: bool
}


#[derive(Debug, Deserialize)]
struct CardInfo {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
    element: Element,
    stars: Stars,
    kind: Kind,
    #[serde(default)]
    desc: String,
    #[serde(flatten)]
    variant: CardVariant
}

impl CardInfo {
    fn make_card(&self) -> Box<dyn Card> {
        match &self.variant {
            CardVariant::BasicCard(data) => {
                Box::new(BasicCard {id: self.id, name: self.name.clone(), element: self.element, stars: self.stars, kind: self.kind, desc: self.desc.clone(),
                     attack: data.attack, heal: data.heal, draw: data.draw, dice: data.dice})
            }
        }
    }
}


//recuperation du deck depuis un fichier JSON
pub fn create_deck_database() -> Vec<Box<dyn Card>> {
    let path = "assets/deck1.json";

    if !Path::new(path).exists() {
        panic!("Fichier JSON non trouvé à : {}", path);
    }

    let file = File::open(path).expect("Impossible d’ouvrir le fichier JSON");
    let reader = BufReader::new(file);

    let deck_data: Vec<CardInfo> = serde_json::from_reader(reader).expect("Erreur de lecture JSON");

    let mut deck: Vec<Box<dyn Card>> = Vec::new();

    for card_info in deck_data.iter() {
        deck.push(card_info.make_card());
    }

    println!("Deck chargé depuis {} !", path);
    for (i, card) in deck.iter().enumerate() {
        println!("Carte {} : {:?}", i + 1, card);
    }

    deck
}
