use std::{thread, time::Duration};
use rand::Rng;
use uuid::Uuid;

use crate::deck;
use crate::card::Card;
use crate::player::Player;

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
        test_random(player, opponent); // récursif avec les mêmes références
        return;
    }

    get_deck(player, opponent);
}

pub fn get_deck(player: &mut Player, opponent: &mut Player) {
    let mut deck: Vec<Card> = deck::create_deck_test();
    deck::shuffle_deck(&mut deck);

    println!("Deck mélangé :");
    for card in deck.iter() {
        println!("{}", card.name);
    }

    give_card2(&deck, player, opponent);
}

pub fn give_card2(deck: &[Card], player: &mut Player, opponent: &mut Player) {
    let player1_cards = &deck[0..5];
    let player2_cards = &deck[5..10];

    println!("Joueur 1 : {:?}", player1_cards);
    println!("Joueur 2 : {:?}", player2_cards);
    println!("Joueur 1 : Cartes en main 5 Cartes défaussées 0");
    println!("Joueur 2 : Cartes en main 5 Cartes défaussées 0");

    if player.order == 1 {
        println!("C'est à votre tour de jouer !");
        player_turn(player1_cards, player2_cards, player, opponent);
    } else {
        println!("C'est au tour de l'adversaire de jouer !");
        opponent_turn(player2_cards, player1_cards, opponent, player);
    }
}

fn player_turn(player1_cards: &[Card], player2_cards: &[Card], player: &mut Player, opponent: &mut Player) {
    println!("Sélectionnez la carte à jouer :");
    for (index, card) in player1_cards.iter().enumerate() {
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
        Ok(n) if n >= 1 && n <= player1_cards.len() => n,
        _ => {
            println!("Choix invalide, veuillez réessayer.");
            return;
        }
    };

    let selected_card = &player1_cards[choix - 1];
    println!("Vous avez sélectionné : {}", selected_card.name);

    play_card(selected_card, player, opponent, player1_cards, player2_cards);
}

fn opponent_turn(player2_cards: &[Card], player1_cards: &[Card], opponent: &mut Player, player: &mut Player) {
    let mut rng = rand::thread_rng();
    let choix = rng.gen_range(0..player2_cards.len());
    println!("L'adversaire a choisi la carte numéro : {}", choix + 1);

    let selected_card = &player2_cards[choix];
    println!("L'adversaire a sélectionné : {}", selected_card.name);

    play_card(selected_card, opponent, player, player2_cards, player1_cards);
}

fn play_card(card: &Card, player: &mut Player, opponent: &mut Player, player_cards: &[Card], opponent_cards: &[Card]) {
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
    }

    if card.heal > 0 {
        player.life += card.heal;
        println!("{} récupère {} points de vie.", player.name, card.heal);
    }

    println!("Vie de {}: {}", player.name, player.life);
    println!("Vie de {}: {}", opponent.name, opponent.life);
     if opponent.life <= 0
     {
    println!("{} est vaincu ! {} remporte la partie !", opponent.name, player.name);
    return; // On arrête ici
    } 
    else if player.life <= 0 
    {
        println!("{} est vaincu ! {} remporte la partie !", player.name, opponent.name);
        return; // Cas très rare mais bon à prévoir
    }
    thread::sleep(Duration::from_millis(500));

    check_order(player_cards, opponent_cards, player, opponent);
   

}

fn check_order(
    player_cards: &[Card],
    opponent_cards: &[Card],
    player: &mut Player,
    opponent: &mut Player
) {
    if player.order == 1 {
        println!("C'est au tour de l'adversaire de jouer !");
        opponent_turn(opponent_cards, player_cards, opponent, player);
    } else {
        println!("C'est à votre tour de jouer !");
        player_turn(opponent_cards, player_cards, opponent, player);
    }
}