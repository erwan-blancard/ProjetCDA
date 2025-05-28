use std::{thread, time::Duration};
use rand::Rng;
use uuid::Uuid;

use crate::deck;
use crate::card::Card;
use crate::player::Player;

use crate::play_info::{PlayInfo, PlayAction, Target, Action};

pub fn test() {
    let mut player = Player {
        id: Uuid::new_v4(),
        name: "Joueur".to_string(),
        life: 100,
        order: 0,
        current_hand_card: Vec::new(),
        current_discard_card: Vec::new(),
        attack_boost: 0,
        effect: String::new(),
        level: 1,
    };

    let mut opponent = Player {
        id: Uuid::new_v4(),
        name: "Adversaire".to_string(),
        life: 100,
        order: 0,
        current_hand_card: Vec::new(),
        current_discard_card: Vec::new(),
        attack_boost: 0,
        effect: String::new(),
        level: 1,
    };

    println!("Vous allez affronter l'ordinateur !");
    thread::sleep(Duration::from_millis(500));
    println!("Préparez-vous !");
    thread::sleep(Duration::from_millis(500));

    test_random(&mut player, &mut opponent);
}

pub fn test_random(player: &mut Player, opponent: &mut Player) {
    let joueur1 = rand::thread_rng().gen_range(1..=6);
    let joueur2 = rand::thread_rng().gen_range(1..=6);

    println!("Joueur 1 a lancé le dé et a obtenu : {}", joueur1);
    println!("Joueur 2 a lancé le dé et a obtenu : {}", joueur2);

    if joueur1 > joueur2 {
        println!("Joueur 1 commence !");
        player.order = 1;
        opponent.order = 2;
    } else if joueur2 > joueur1 {
        println!("Joueur 2 commence !");
        player.order = 2;
        opponent.order = 1;
    } else {
        println!("Égalité, relancez le dé !");
        test_random(player, opponent);
        return;
    }

    get_deck_database(player, opponent);
}
pub fn get_deck_database(player: &mut Player, opponent: &mut Player) {
    let mut deck = deck::create_deck_database();
    deck::shuffle_deck(&mut deck);

    give_card2(&mut deck, player, opponent);
}
pub fn get_deck(player: &mut Player, opponent: &mut Player) {
    let mut deck: Vec<Card> = deck::create_deck_test();
    deck::shuffle_deck(&mut deck);

    println!("Deck mélangé :");
    for card in deck.iter() {
        println!("{}", card.name);
    }

    give_card2(&mut deck, player, opponent);
}

pub fn give_card2(deck: &mut Vec<Card>, player: &mut Player, opponent: &mut Player) {
    player.current_hand_card = deck[0..5].to_vec();
    opponent.current_hand_card = deck[5..10].to_vec();

    println!("Joueur 1 : {:?}", player.current_hand_card);
    println!("Joueur 2 : {:?}", opponent.current_hand_card);
    println!("Joueur 1 : Cartes en main 5 Cartes défaussées 0");
    println!("Joueur 2 : Cartes en main 5 Cartes défaussées 0");

    let mut is_player_turn = player.order == 1;

    loop {
        if is_player_turn {
            println!("\nTour du joueur !");
            if player_turn(player, opponent, deck) {
                break;
            }
        } else {
            println!("\nTour de l’adversaire !");
            if opponent_turn(opponent, player, deck) {
                break;
            }
        }

        is_player_turn = !is_player_turn;
    }
}

fn player_turn(player: &mut Player, opponent: &mut Player, deck: &mut Vec<Card>) -> bool {
    let to_draw = 5usize.saturating_sub(player.current_hand_card.len());
    if to_draw > 0 
    {
    println!("{} pioche {} carte(s).", player.name, to_draw);
    draw_cards(player, to_draw, deck);
    }
    println!("Sélectionnez la carte à jouer :");
    for (index, card) in player.current_hand_card.iter().enumerate() {
        println!(
            "{}. {} - Element: {:?} Nature: {:?} Stars: {:?}",
            index + 1,
            card.name,
            card.element,
            card.nature,
            card.stars
        );
    }

    let mut choix = String::new();
    std::io::stdin().read_line(&mut choix).expect("Échec de la lecture");

    let choix: usize = match choix.trim().parse() {
        Ok(n) if n >= 1 && n <= player.current_hand_card.len() => n,
        _ => {
            println!("Choix invalide, veuillez réessayer.");
            return false;
        }
    };

    let selected_card = player.current_hand_card[choix - 1].clone();
    println!("Vous avez sélectionné : {}", selected_card.name);

    play_card(&selected_card, player, opponent, deck);

    discard_card(&selected_card, player);

    println!(
        "Cartes en main: {}, Cartes défaussées: {}",
        player.current_hand_card.len(),
        player.current_discard_card.len()
    );

    player.life <= 0 || opponent.life <= 0
}

fn opponent_turn(opponent: &mut Player, player: &mut Player, deck: &mut Vec<Card>) -> bool {
    //verification de la main de l'adversaire
    let to_draw = 5usize.saturating_sub(opponent.current_hand_card.len());
    if to_draw > 0 
    {
    println!("{} pioche {} carte(s).", opponent.name, to_draw);
    draw_cards(opponent, to_draw, deck);
    }
    let mut rng = rand::thread_rng();
    let choix = rng.gen_range(0..opponent.current_hand_card.len());
    let selected_card = opponent.current_hand_card[choix].clone();
    
    println!("L'adversaire a sélectionné : {}", selected_card.name);

    play_card(&selected_card, opponent, player, deck);

    discard_card(&selected_card, opponent);

    opponent.life <= 0 || player.life <= 0
}

fn discard_card(card: &Card, player: &mut Player) {
    if let Some(pos) = player.current_hand_card.iter().position(|c| c.id == card.id) {
        let removed_card = player.current_hand_card.remove(pos);
        player.current_discard_card.push(removed_card);
        println!("Carte défaussée : {}", card.name);
    } else {
        println!("Erreur : carte non trouvée dans la main.");
    }

    thread::sleep(Duration::from_millis(500));
}

fn play_card(card: &Card, player: &mut Player, opponent: &mut Player, deck: &mut Vec<Card>) {
    thread::sleep(Duration::from_millis(500));

    if card.attack > 0 {
        let total_attack = card.attack + player.attack_boost;
        opponent.life -= total_attack;
        println!(
            "{} attaque {} et inflige {} points de dégâts.",
            player.name, opponent.name, total_attack
        );
    }

    if card.draw > 0 {
        println!("{} pioche {} carte(s).", player.name, card.draw);
        draw_cards(player, card.draw as usize, deck);
    }

    if card.heal > 0 {
        player.life += card.heal;
        println!("{} récupère {} points de vie.", player.name, card.heal);
    }
    let mut roll = 0;
    if card.dice == true
     {
        roll = rand::thread_rng().gen_range(1..=6);
        let total_attack = card.attack + roll + player.attack_boost;

        opponent.life -= total_attack;
        println!("{} inflige {} dégâts à l'adversaire ! ({} + {})", card.name, total_attack, card.attack+ player.attack_boost, roll);
        
    }

    println!("Vie de {}: {}", player.name, player.life);
    println!("Vie de {}: {}", opponent.name, opponent.life);

    if opponent.life <= 0 {
        println!("{} est vaincu ! {} remporte la partie !", opponent.name, player.name);
    } else if player.life <= 0 {
        println!("{} est vaincu ! {} remporte la partie !", player.name, opponent.name);
    }
    //recuperer information sur le tour de jeu grace aux structs contenue dans la classe play_info
   
let action = Action::Attack { amount: card.attack as u32};

let target = Target {
    player_id: opponent.id,
    action,
    effect: card.name.clone(),
};

let play_action = PlayAction {
    dice_roll: roll as u8,
    targets: vec![target],
    player_dice_id: player.id,
};

let play_info = PlayInfo {
    actions: vec![play_action],
};

println!("Résumé du tour : {:?}", play_info);


    thread::sleep(Duration::from_millis(500));
}

fn draw_cards(player: &mut Player, count: usize, deck: &mut Vec<Card>) {
    for _ in 0..count {
        if let Some(card) = deck.pop() {
            println!("{} a pioché : {}", player.name, card.name);
            player.current_hand_card.push(card);
        } else {
            println!("Le deck est vide ! Impossible de piocher.");
            break;
        }
    }
}
