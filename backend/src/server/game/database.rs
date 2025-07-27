use std::{fs::File, io::BufReader, path::Path};
use std::fmt;

use lazy_static::lazy_static;
use serde::Deserializer;
use serde::{de::{SeqAccess, Visitor}, Deserialize};

use super::modifiers::ModifierInfo;
use super::buffs::BuffVariant;

use super::cards::card::{BasicCard, Card, CardId, Element, Kind, Stars, TargetType};
use super::cards::multi_action_card::MultiActionCard;
use super::cards::multi_hit_card::MultiHitCard;
use super::cards::pearth_card::PearthCard;
use super::cards::players_rolls_dice_card::{PlayersRollsDiceCard, PlayersRollsDiceCardAction};
use super::cards::target_both_card::TargetBothCard;


lazy_static! {
    pub static ref CARD_DATABASE: Vec<Box<dyn Card>> = {
        get_card_database()
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
    PearthCard,
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
    #[serde(default)]
    buffs: Vec<BuffVariant>,
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
                    buffs: self.buffs.clone().into_iter().map(|b| b.into_boxed()).collect(),
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
                    buffs: self.buffs.clone().into_iter().map(|b| b.into_boxed()).collect(),
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
                    buffs: self.buffs.clone().into_iter().map(|b| b.into_boxed()).collect(),
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
                    buffs: self.buffs.clone().into_iter().map(|b| b.into_boxed()).collect(),
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
                    buffs: self.buffs.clone().into_iter().map(|b| b.into_boxed()).collect(),
                })
            }
            CardVariant::PearthCard => {
                Box::new(PearthCard {
                    id: self.id,
                    name: self.name.clone(),
                    element: self.element,
                    stars: self.stars,
                    kind: self.kind,
                    desc: self.desc.clone(),
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


pub fn get_card_database() -> Vec<Box<dyn Card>> {
    let path = std::env::var("CARDS_FILE_PATH").expect("CARDS_FILE_PATH not set !");

    if !Path::new(&path).exists() {
        panic!("JSON file for cards not found ({})", path);
    }

    let file = File::open(&path).expect("Could not open JSON file");
    let reader = BufReader::new(file);

    let cards_info: CardInfoList = serde_json::from_reader(reader).expect("Error reading JSON file");

    let mut deck: Vec<Box<dyn Card>> = Vec::new();

    for card_info in cards_info.0.iter() {
        deck.push(card_info.make_card());
    }

    for (i, card) in deck.iter().enumerate() {
        println!("Card {} : {:?}", i + 1, card);
    }

    deck
}
