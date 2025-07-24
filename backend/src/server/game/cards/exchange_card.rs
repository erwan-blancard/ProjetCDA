use std::collections::HashSet;

use super::card::{Card, CardId, Element, Kind, Stars, TargetType};
use super::super::modifiers::Modifier;
use super::super::buffs::{Buff, BuffType};
use super::super::game::{Game, MAX_PLAYERS};
use super::super::player::Player;
use super::super::play_info::{PlayAction, PlayInfo, ActionTarget, ActionType};

use crate::server::game::cards::card::check_apply_attack_buffs;
use crate::utils::clamp::clamp;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use crate::server::game::cards::card::BasicCard;
use rand::prelude::SliceRandom;


#[derive(Debug, Clone)]
pub enum ComplexEffect {
    Steal { count: usize, element: Option<String>, kind: Option<String>, stars: Option<String> },
    Give,
    Exchange,
}

impl<'de> Deserialize<'de> for ComplexEffect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(obj) = value.as_object() {
            match obj.get("type").and_then(|v| v.as_str()) {
                Some("steal") => {
                    let count = obj.get("count").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
                    let element = obj.get("element").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let kind = obj.get("kind").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let stars = obj.get("stars").and_then(|v| v.as_str()).map(|s| s.to_string());
                    Ok(ComplexEffect::Steal { count, element, kind, stars })
                }
                Some("give") => Ok(ComplexEffect::Give),
                Some("exchange") => Ok(ComplexEffect::Exchange),
                _ => Err(serde::de::Error::custom("Unknown effect type in object")),
            }
        } else {
            Err(serde::de::Error::custom("Invalid effect format: expected object"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComplexEffectCard {
    pub base: BasicCard,
    pub complex_effects: Vec<ComplexEffect>,
}

impl Card for ComplexEffectCard {
    fn get_id(&self) -> CardId { self.base.get_id() }
    fn get_name(&self) -> String { self.base.get_name() }
    fn get_attack(&self) -> u32 { self.base.get_attack() }
    fn get_attack_modifier(&self) -> Option<Box<dyn Modifier>> { self.base.get_attack_modifier() }
    fn get_heal(&self) -> u32 { self.base.get_heal() }
    fn get_heal_modifier(&self) -> Option<Box<dyn Modifier>> { self.base.get_heal_modifier() }
    fn get_draw(&self) -> u32 { self.base.get_draw() }
    fn get_draw_modifier(&self) -> Option<Box<dyn Modifier>> { self.base.get_draw_modifier() }
    fn get_description(&self) -> String { self.base.get_description() }
    fn get_kind(&self) -> Kind { self.base.get_kind() }
    fn get_element(&self) -> Element { self.base.get_element() }
    fn get_stars(&self) -> Stars { self.base.get_stars() }
    fn get_target_type(&self) -> TargetType { self.base.get_target_type() }

    fn play(&self, player_index: usize, target_indices: Vec<usize>, game: &mut Game) -> Result<(PlayInfo, HashSet<usize>), String> {
        let mut info: PlayInfo = PlayInfo::new();
        let mut buffs_used: HashSet<usize> = HashSet::new();
        let targets = target_indices.iter().map(|i| &game.players[*i]).collect();
        self.base.validate_targets(&targets)?;

        // Appliquer les effets de base (attaque, heal, draw) comme dans BasicCard::play
        let dice_roll = rand::random_range(0..6) + 1;
        let mut dice_roll_used = false;
        // Attaque
        if self.base.get_attack() > 0 || self.base.get_attack_modifier().is_some() {
            for &target_index in &target_indices {
                let mut attack_action: PlayAction = PlayAction::new();
                let (player, target) = if player_index < target_index {
                    let (left, right) = game.players.split_at_mut(target_index);
                    (&mut left[player_index], &mut right[0])
                } else if player_index > target_index {
                    let (left, right) = game.players.split_at_mut(player_index);
                    (&mut right[0], &mut left[target_index])
                } else {
                    return Err("Target is player !".to_string());
                };
                let (amount, dice_roll_val, player_dice_id) = {
                    if let Some(modifier) = self.base.get_attack_modifier() {
                        modifier.compute(self.base.get_attack(), player, target, Some(dice_roll))
                    } else { (self.base.get_attack(), 0, -1) }
                };
                if !dice_roll_used && player_dice_id != -1 {
                    attack_action.dice_roll = dice_roll_val;
                    attack_action.player_dice_id = player_dice_id;
                    dice_roll_used = true;
                }
                let amount = check_apply_attack_buffs(amount, &player.buffs, self.base.get_element(), self.base.get_kind(), self.base.get_stars(), &mut buffs_used);
                let action_target = target.damage(amount, self.base.get_damage_effect());
                attack_action.targets.push(action_target);
                info.actions.push(attack_action);
            }
        }
        // Heal
        if self.base.get_heal() > 0 || self.base.get_heal_modifier().is_some() {
            let player = &mut game.players[player_index];
            let mut heal_action: PlayAction = PlayAction::new();
            let (amount, dice_roll_val, player_dice_id) = {
                if let Some(modifier) = self.base.get_heal_modifier() {
                    modifier.compute(self.base.get_heal(), player, player, Some(dice_roll))
                } else { (self.base.get_heal(), 0, -1) }
            };
            if !dice_roll_used && player_dice_id != -1 {
                heal_action.dice_roll = dice_roll_val;
                heal_action.player_dice_id = player_dice_id;
                dice_roll_used = true;
            }
            let action_target = player.heal(amount, self.base.get_heal_effect());
            heal_action.targets.push(action_target);
            info.actions.push(heal_action);
        }
        // Draw
        if self.base.get_draw() > 0 || self.base.get_draw_modifier().is_some() {
            let player = &mut game.players[player_index];
            let (amount, dice_roll_val, player_dice_id) = {
                if let Some(modifier) = self.base.get_draw_modifier() {
                    modifier.compute(self.base.get_draw(), player, player, Some(dice_roll))
                } else { (self.base.get_draw(), 0, -1) }
            };
            let drawn_cards = Game::give_from_pile(&mut game.pile, player, amount as usize);
            if drawn_cards.len() > 0 {
                let mut draw_action = PlayAction::new();
                if !dice_roll_used && player_dice_id != -1 {
                    draw_action.dice_roll = dice_roll_val;
                    draw_action.player_dice_id = player_dice_id;
                    dice_roll_used = true;
                }
                draw_action.targets.push(ActionTarget {
                    player_id: player.id,
                    action: ActionType::Draw { cards: drawn_cards },
                    effect: String::new()
                });
                info.actions.push(draw_action);
            }
        }
        // Appliquer les effets complexes
        for effect in &self.complex_effects {
            match effect {
                ComplexEffect::Steal { count, element, kind, stars } => {
                    // Cas spécial : tornade (vole 2 cartes aléatoires et inflige 8 dégâts)
                    if let Some(name) = Some(self.base.get_name()) {
                        if name.to_lowercase() == "tornade" {
                            let target_index = target_indices[0];
                            let (player, target) = if player_index < target_index {
                                let (left, right) = game.players.split_at_mut(target_index);
                                (&mut left[player_index], &mut right[0])
                            } else if player_index > target_index {
                                let (left, right) = game.players.split_at_mut(player_index);
                                (&mut right[0], &mut left[target_index])
                            } else {
                                return Err("Target is player !".to_string());
                            };
                            // Infliger 8 points de dégâts
                            let damage = 8;
                            let action_target = target.damage(damage, "tornade_damage".to_string());
                            let mut attack_action = PlayAction::new();
                            attack_action.targets.push(action_target);
                            info.actions.push(attack_action);
                            // Voler 2 cartes aléatoires
                            let stolen_ids = steal_n_random_cards(&mut target.hand_cards, &mut player.hand_cards, 2);
                            if !stolen_ids.is_empty() {
                                println!("[TORNADE] Player {} steals card ids {:?} from player {}", player_index, stolen_ids, target_index);
                                let mut steal_action = PlayAction::new();
                                steal_action.targets.push(ActionTarget {
                                    player_id: player_index as i32,
                                    action: ActionType::Steal { cards: stolen_ids },
                                    effect: "steal".to_string(),
                                });
                                info.actions.push(steal_action);
                            } else {
                                println!("[TORNADE] No card in hand for player {}", target_index);
                            }
                            continue;
                        }
                    }
                    let target_index = target_indices[0];
                    let target_hand = &game.players[target_index].hand_cards;
                    if !has_card_matching_filter(target_hand, element, kind, stars) {
                        println!("[STEAL] No matching card to steal for filter: element={:?}, kind={:?}, stars={:?}", element, kind, stars);
                        continue;
                    }
                    let mut stolen_ids = Vec::new();
                    let mut to_steal = *count;
                    while to_steal > 0 {
                        let filtered_indices: Vec<usize> = game.players[target_index].hand_cards.iter()
                            .enumerate()
                            .filter(|(_, card)| {
                                let mut ok = true;
                                if let Some(ref el) = element {
                                    ok &= card.get_element().to_string().to_lowercase() == el.to_lowercase();
                                }
                                if let Some(ref k) = kind {
                                    ok &= card.get_kind().to_string().to_lowercase() == k.to_lowercase();
                                }
                                if let Some(ref s) = stars {
                                    ok &= card.get_stars().to_string().to_lowercase() == s.to_lowercase();
                                }
                                ok
                            })
                            .map(|(i, _)| i)
                            .collect();
                        if filtered_indices.is_empty() {
                            break;
                        }
                        let mut rng = rand::thread_rng();
                        let mut shuffled = filtered_indices.clone();
                        shuffled.shuffle(&mut rng);
                        let i = shuffled[0];
                        let card = game.players[target_index].hand_cards.remove(i);
                        game.players[player_index].hand_cards.push(card.clone_box());
                        stolen_ids.push(card.get_id());
                        to_steal -= 1;
                    }
                    println!("[STEAL] Player {} steals card ids {:?} from player {}", player_index, stolen_ids, target_index);
                    let mut steal_action = PlayAction::new();
                    steal_action.targets.push(ActionTarget {
                        player_id: player_index as i32,
                        action: ActionType::Steal { cards: stolen_ids },
                        effect: "steal".to_string(),
                    });
                    info.actions.push(steal_action);
                }
                ComplexEffect::Give => {
                    let card_name = self.base.get_name().to_lowercase();
                    let target_index = target_indices[0];
                    let (player, target) = if player_index < target_index {
                        let (left, right) = game.players.split_at_mut(target_index);
                        (&mut left[player_index], &mut right[0])
                    } else if player_index > target_index {
                        let (left, right) = game.players.split_at_mut(player_index);
                        (&mut right[0], &mut left[target_index])
                    } else {
                        return Err("Target is player !".to_string());
                    };
                    // Pour Flambeau, on ne défausse pas la carte, on la transfère au joueur ciblé
                    if card_name == "flambeau" {
                        // 1. Infliger les dégâts
                        let damage = self.base.get_attack();
                        if damage > 0 {
                            let mut attack_action = PlayAction::new();
                            let action_target = target.damage(damage, "flambeau_damage".to_string());
                            attack_action.targets.push(action_target);
                            info.actions.push(attack_action);
                        }
                        // 2. Transférer la carte Flambeau au joueur ciblé
                        let pos = player.hand_cards.iter().position(|c| c.get_name().to_lowercase() == "flambeau");
                        if let Some(idx) = pos {
                            let card = player.hand_cards.remove(idx);
                            let id = card.get_id();
                            target.hand_cards.push(card);
                            // On log l'action de don
                            let mut give_action = PlayAction::new();
                            give_action.targets.push(ActionTarget {
                                player_id: target_index as i32,
                                action: ActionType::Steal { cards: vec![id] },
                                effect: "give".to_string(),
                            });
                            info.actions.push(give_action);
                        }
                        // 3. Pioche une carte
                        let drawn_cards = Game::give_from_pile(&mut game.pile, player, 1);
                        if !drawn_cards.is_empty() {
                            let mut draw_action = PlayAction::new();
                            draw_action.targets.push(ActionTarget {
                                player_id: player_index as i32,
                                action: ActionType::Draw { cards: drawn_cards },
                                effect: "draw".to_string(),
                            });
                            info.actions.push(draw_action);
                        }
                    } else if card_name == "airtichaut" {
                        // Airtichaut : soigne, donne toutes les autres cartes sauf elle-même
                        let nb_cartes = player.hand_cards.len();
                        if nb_cartes > 0 {
                            let heal = nb_cartes * 5;
                            let mut heal_action = PlayAction::new();
                            let action_target = player.heal(heal as u32, "airtichaut_heal".to_string());
                            heal_action.targets.push(action_target);
                            info.actions.push(heal_action);
                        }
                        // Donner toutes les autres cartes au ciblé (sauf la carte airtichaut elle-même)
                        // On identifie la carte airtichaut par son nom (en minuscule)
                        let pos = player.hand_cards.iter().position(|c| c.get_name().to_lowercase() == "airtichaut");
                        let mut to_give = Vec::new();
                        for (i, card) in player.hand_cards.iter().enumerate() {
                            if Some(i) != pos {
                                to_give.push(card.get_id());
                            }
                        }
                        let mut given_ids = Vec::new();
                        // On retire les cartes à donner en partant du plus grand index pour ne pas décaler les indices
                        to_give.sort_unstable_by(|a, b| b.cmp(a));
                        for id in to_give {
                            if let Some(idx) = player.hand_cards.iter().position(|c| c.get_id() == id) {
                                let card = player.hand_cards.remove(idx);
                                given_ids.push(card.get_id());
                                target.hand_cards.push(card);
                            }
                        }
                        if !given_ids.is_empty() {
                            let mut give_action = PlayAction::new();
                            give_action.targets.push(ActionTarget {
                                player_id: target_index as i32,
                                action: ActionType::Steal { cards: given_ids },
                                effect: "give".to_string(),
                            });
                            info.actions.push(give_action);
                        }
                    } else if card_name == "shuriken" {
                        // Shuriken : donne 2 cartes au ciblé, puis pioche 2 cartes
                        use rand::seq::SliceRandom;
                        let mut rng = rand::thread_rng();
                        let hand_len = player.hand_cards.len();
                        let mut indices: Vec<usize> = (0..hand_len).collect();
                        indices.shuffle(&mut rng);
                        let mut to_remove: Vec<usize> = indices.into_iter().take(2).collect();
                        to_remove.sort_unstable_by(|a, b| b.cmp(a)); // du plus grand au plus petit
                        let mut given_ids = Vec::new();
                        for i in to_remove {
                            if i < player.hand_cards.len() {
                                let card = player.hand_cards.remove(i);
                                given_ids.push(card.get_id());
                                target.hand_cards.push(card);
                            }
                        }
                        if !given_ids.is_empty() {
                            let mut give_action = PlayAction::new();
                            give_action.targets.push(ActionTarget {
                                player_id: target_index as i32,
                                action: ActionType::Steal { cards: given_ids },
                                effect: "give".to_string(),
                            });
                            info.actions.push(give_action);
                        }
                        // Pioche 2 cartes
                        let drawn_cards = Game::give_from_pile(&mut game.pile, player, 2);
                        if !drawn_cards.is_empty() {
                            let mut draw_action = PlayAction::new();
                            draw_action.targets.push(ActionTarget {
                                player_id: player_index as i32,
                                action: ActionType::Draw { cards: drawn_cards },
                                effect: "draw".to_string(),
                            });
                            info.actions.push(draw_action);
                        }
                    } else {
                        // Défausser la carte jouée (celle qui déclenche le don)
                        let pos = player.hand_cards.iter().position(|c| c.get_name().to_lowercase() == card_name);
                        if let Some(idx) = pos {
                            let _ = player.hand_cards.remove(idx);
                        }
                        // Donner toutes les autres cartes au ciblé
                        let mut to_give = Vec::new();
                        for i in (0..player.hand_cards.len()).rev() {
                            to_give.push(player.hand_cards.remove(i));
                        }
                        let mut given_ids = Vec::new();
                        for card in to_give {
                            given_ids.push(card.get_id());
                            target.hand_cards.push(card);
                        }
                        if !given_ids.is_empty() {
                            let mut give_action = PlayAction::new();
                            give_action.targets.push(ActionTarget {
                                player_id: target_index as i32,
                                action: ActionType::Steal { cards: given_ids },
                                effect: "give".to_string(),
                            });
                            info.actions.push(give_action);
                        }
                    }
                }
                &ComplexEffect::Exchange => {
                    // TODO: implémenter la mécanique d'échange si besoin
                    todo!("Effet Exchange non encore implémenté");
                }
            }
        }
        Ok((info, buffs_used))
    }
}

// Fonction utilitaire pour vérifier si une main contient au moins une carte filtrée
fn has_card_matching_filter(hand: &Vec<Box<dyn Card>>, element: &Option<String>, kind: &Option<String>, stars: &Option<String>) -> bool {
    hand.iter().any(|card| {
        let mut ok = true;
        if let Some(ref el) = element {
            ok &= card.get_element().to_string().to_lowercase() == el.to_lowercase();
        }
        if let Some(ref k) = kind {
            ok &= card.get_kind().to_string().to_lowercase() == k.to_lowercase();
        }
        if let Some(ref s) = stars {
            ok &= card.get_stars().to_string().to_lowercase() == s.to_lowercase();
        }
        ok
    })
}

// Fonction utilitaire : vérifie si la main contient au moins une carte d'élément Air
fn has_air_card(hand: &Vec<Box<dyn Card>>) -> bool {
    hand.iter().any(|card| card.get_element().to_string().to_lowercase() == "air")
}

// Fonction utilitaire : vole jusqu'à n cartes aléatoires de from vers to, retourne la liste des ids volés
fn steal_n_random_cards(from: &mut Vec<Box<dyn Card>>, to: &mut Vec<Box<dyn Card>>, n: usize) -> Vec<CardId> {
    use rand::seq::SliceRandom;
    let mut stolen_ids = Vec::new();
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..from.len()).collect();
    indices.shuffle(&mut rng);
    for &i in indices.iter().take(n) {
        if i < from.len() {
            let card = from.remove(i);
            let id = card.get_id();
            to.push(card);
            stolen_ids.push(id);
        }
    }
    stolen_ids
}