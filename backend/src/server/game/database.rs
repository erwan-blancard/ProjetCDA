use std::{fs::File, io::BufReader, path::Path};
use std::fmt;

use diesel::expression::is_aggregate::No;
use lazy_static::lazy_static;
use serde::Deserializer;
use serde::{de::{SeqAccess, Visitor}, Deserialize};

use crate::server::game::card::{MultiActionCard, MultiHitCard, TargetBothCard, TargetType};
use crate::server::game::modifiers::{Modifier, ModifierInfo};
use crate::server::game::special_cards::{PlayersRollsDiceCard, PlayersRollsDiceCardAction, PlayersRollsDiceCardActionType};

use super::card::{BasicCard, Card, CardId, Element, Kind, Stars};

lazy_static! {
    pub static ref CARD_DATABASE: Vec<Box<dyn Card>> = {
        create_deck_database()
    };
}


#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum CardVariant {
    BasicCard(BasicCardData),
    MultiHitCard(MultiHitCardData),
    TargetBothCard(BasicCardData),    // same fields as BasicCard
    MultiActionCard(MultiActionCardData),
    PlayersRollsDiceCard(PlayersRollsDiceCardData),
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
    attack_modifier: Option<ModifierInfo>,
    #[serde(default)]
    heal_modifier: Option<ModifierInfo>,
    #[serde(default)]
    draw_modifier: Option<ModifierInfo>,
    #[serde(default)]
    targets: TargetType,
}


#[derive(Debug, Deserialize)]
struct MultiHitCardData {
    attacks: Vec<u32>,
    #[serde(default)]
    heal: u32,
    #[serde(default)]
    draw: u32,
    #[serde(default)]
    heal_modifier: Option<ModifierInfo>,
    #[serde(default)]
    draw_modifier: Option<ModifierInfo>,
}


#[derive(Debug, Deserialize)]
struct MultiActionCardData {
    actions: usize,
    #[serde(default)]
    targets: Vec<TargetType>,
    #[serde(default)]
    attacks: Vec<u32>,
    #[serde(default)]
    heals: Vec<u32>,
    #[serde(default)]
    draws: Vec<u32>,
    #[serde(default)]
    attack_modifiers: Vec<Option<ModifierInfo>>,
    #[serde(default)]
    heal_modifiers: Vec<Option<ModifierInfo>>,
    #[serde(default)]
    draw_modifiers: Vec<Option<ModifierInfo>>,
}


#[derive(Debug, Deserialize)]
struct PlayersRollsDiceCardData {
    #[serde(default)]
    attack: bool,
    #[serde(default)]
    heal: bool,
    #[serde(default)]
    draw: bool,
    #[serde(default)]
    targets: TargetType,
    dice_action: PlayersRollsDiceCardAction,
}


/// Common card data
#[derive(Debug, Deserialize)]
struct CardInfo {
    #[serde(default)]
    id: CardId,
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
                Box::new(BasicCard {
                    id: self.id,
                    name: self.name.clone(),
                    element: self.element,
                    stars: self.stars,
                    kind: self.kind,
                    desc: self.desc.clone(),
                    attack: data.attack,
                    heal: data.heal,
                    draw: data.draw,
                    attack_modifier: data.attack_modifier.clone().map(|m| m.into_boxed()),
                    heal_modifier: data.heal_modifier.clone().map(|m| m.into_boxed()),
                    draw_modifier: data.draw_modifier.clone().map(|m| m.into_boxed()),
                    target_type: data.targets,
                })
            },
            CardVariant::MultiHitCard(data) => {
                Box::new(MultiHitCard {
                    id: self.id,
                    name: self.name.clone(),
                    element: self.element,
                    stars: self.stars,
                    kind: self.kind,
                    desc: self.desc.clone(),
                    attacks: data.attacks.clone(),
                    heal: data.heal,
                    draw: data.draw,
                    heal_modifier: data.heal_modifier.clone().map(|m| m.into_boxed()),
                    draw_modifier: data.draw_modifier.clone().map(|m| m.into_boxed()),
                })
            },
            CardVariant::TargetBothCard(data) => {
                Box::new(TargetBothCard {
                    id: self.id,
                    name: self.name.clone(),
                    element: self.element,
                    stars: self.stars,
                    kind: self.kind,
                    desc: self.desc.clone(),
                    attack: data.attack,
                    heal: data.heal,
                    draw: data.draw,
                    attack_modifier: data.attack_modifier.clone().map(|m| m.into_boxed()),
                    heal_modifier: data.heal_modifier.clone().map(|m| m.into_boxed()),
                    draw_modifier: data.draw_modifier.clone().map(|m| m.into_boxed()),
                    target_type: data.targets,
                })
            }
            CardVariant::MultiActionCard(data) => {
                Box::new(MultiActionCard {
                    id: self.id,
                    name: self.name.clone(),
                    element: self.element,
                    stars: self.stars,
                    kind: self.kind,
                    desc: self.desc.clone(),
                    actions: data.actions,
                    attacks: data.attacks.clone(),
                    heals: data.heals.clone(),
                    draws: data.draws.clone(),
                    attack_modifiers: data.attack_modifiers.clone().into_iter().map(|m| m.map(|m| m.into_boxed())).collect(),
                    heal_modifiers: data.heal_modifiers.clone().into_iter().map(|m| m.map(|m| m.into_boxed())).collect(),
                    draw_modifiers: data.draw_modifiers.clone().into_iter().map(|m| m.map(|m| m.into_boxed())).collect(),
                    target_types: data.targets.clone(),
                })
            }
            CardVariant::PlayersRollsDiceCard(data) => {
                Box::new(PlayersRollsDiceCard {
                    id: self.id,
                    name: self.name.clone(),
                    element: self.element,
                    stars: self.stars,
                    kind: self.kind,
                    desc: self.desc.clone(),
                    attack: data.attack,
                    heal: data.heal,
                    draw: data.draw,
                    target_type: data.targets,
                    dice_action: data.dice_action,
                })
            }
        }
    }
}


struct CardInfoList(Vec<CardInfo>);

impl<'de> Deserialize<'de> for CardInfoList {
    fn deserialize<D>(deserializer: D) -> Result<CardInfoList, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CardInfoVisitor;

        impl<'de> Visitor<'de> for CardInfoVisitor {
            type Value = CardInfoList;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("list of CardInfo")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<CardInfoList, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut card_info_list = Vec::new();
                let mut idx = 0;

                while let Some(mut p) = seq.next_element::<CardInfo>()? {
                    p.id = idx;
                    idx += 1;
                    card_info_list.push(p);
                }

                Ok(CardInfoList(card_info_list))
            }
        }

        deserializer.deserialize_seq(CardInfoVisitor)
    }
}


//recuperation du deck depuis un fichier JSON
pub fn create_deck_database() -> Vec<Box<dyn Card>> {
    let path = "assets/cards.json";

    if !Path::new(path).exists() {
        panic!("Fichier JSON non trouvé à : {}", path);
    }

    let file = File::open(path).expect("Impossible d’ouvrir le fichier JSON");
    let reader = BufReader::new(file);

    let deck_data: CardInfoList = serde_json::from_reader(reader).expect("Erreur de lecture JSON");

    let mut deck: Vec<Box<dyn Card>> = Vec::new();

    for card_info in deck_data.0.iter() {
        deck.push(card_info.make_card());
    }

    println!("Deck chargé depuis {} !", path);
    for (i, card) in deck.iter().enumerate() {
        println!("Carte {} : {:?}", i + 1, card);
    }

    deck
}
