//gestion des differentes actions
use crate::card;
use rand::random;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use crate::card::Card;
//creer un deck
pub fn create_deck()
{
    println!("création d'un deck");

}
//faire une methode qui renvoie une liste de carte

pub fn create_deck_test() -> Vec<Card>
{
    
    //ajouter la carte dans une liste de carte
    //exemple de carte
    //creer une liste de carte
    let mut deck: Vec<card::Card> = Vec::new();
    //ajouter la carte dans le deck
    deck.push(card::Card::new(
        String::from("Boule de feu"),
        card::Element::Fire,
        card::Stars::Two,
        card::Kind::Spell,
        String::from("Inflige 4 points de dégâts."),
        4,
        0,
        0,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Pomme"),
        card::Element::Earth,
        card::Stars::Two,
        card::Kind::Food,
        String::from("Récupérez 6 points."),
        0,
        6,
        0,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Pêche"),
        card::Element::Water,
        card::Stars::Two,
        card::Kind::Food,
        String::from("Récupérez 4 points. Piochez 1 carte."),
        0,
        4,
        1,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Arbalète"),
        card::Element::Air,
        card::Stars::Four,
        card::Kind::Weapon,
        String::from("Enlevez 9 points de vie à votre adversaire. Piochez 1 carte."),
        9,
        0,
        1,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Bille de feu"),
        card::Element::Fire,
        card::Stars::One,
        card::Kind::Spell,
        String::from("Enlevez 1 point de vie à votre adversaire. "),
        1,
        0,
        0,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Bulle"),
        card::Element::Water,
        card::Stars::One,
        card::Kind::Spell,
        String::from("Lancé un dé. Enlevez le résultat du dé à votre adversaire."),
        0,
        0,
        0,
        true,
    ));
    deck.push(card::Card::new(
        String::from("Pioche"),
        card::Element::Earth,
        card::Stars::Two,
        card::Kind::Weapon,
        String::from("Enlevez 6 points de vie à votre adversaire. Piochez 1 carte."),
        6,
        0,
        1,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Pomme d'amour"),
        card::Element::Fire,
        card::Stars::Two,
        card::Kind::Food,
        String::from("Vous ainsi qu'un autre joueur récupérez 12 points de vie."),
        0,
        12,
        0,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Fléchettes"),
        card::Element::Air,
        card::Stars::One,
        card::Kind::Weapon,
        String::from("Enlevez 3 points de vie à votre adversaire. Piochez 1 carte."),
        3,
        0,
        1,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Grain de poussière"),
        card::Element::Earth,
        card::Stars::One,
        card::Kind::Spell,
        String::from("Enlevez 0 point de vie à votre adversaire."),
        3,
        0,
        1,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Noix de coco"),
        card::Element::Water,
        card::Stars::Four,
        card::Kind::Food,
        String::from("Récupérez 4 points et enlevez 9 points de vie à votre adversaire."),
        9,
        4,
        0,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Epuisette"),
        card::Element::Water,
        card::Stars::Two,
        card::Kind::Weapon,
        String::from("Enlevez 2 points à votre adversaire. Piochez 2 carte."),
        2,
        0,
        2,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Bouteille d'eau"),
        card::Element::Water,
        card::Stars::Three,
        card::Kind::Weapon,
        String::from("Récupérez 8 points. Enlevez 4 points à votre adversaire. Piochez 2 carte."),
        4,
        8,
        2,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Lance de flammes"),
        card::Element::Fire,
        card::Stars::Five,
        card::Kind::Weapon,
        String::from("Enlevez 18 points de vie à votre adversaire."),
        18,
        0,
        0,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Terrassement"),
        card::Element::Earth,
        card::Stars::Five,
        card::Kind::Spell,
        String::from("Enlevez 15 points de vie à votre adversaire. Piochez 2 cartes."),
        15,
        0,
        2,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Vague vampirique"),
        card::Element::Water,
        card::Stars::Five,
        card::Kind::Spell,
        String::from("Enlevez 12 points de vie à votre adversaire. Récupérez 6 points."),
        12,
        6,
        0,
        false,
    ));
    deck.push(card::Card::new(
        String::from("Rafale"),
        card::Element::Air,
        card::Stars::Five,
        card::Kind::Spell,
        String::from("Enlevez 14 points de vie à votre adversaire. Piochez 3 cartes."),
        14,
        0,
        3,
        false,
    ));
    for i in 0..deck.len() {
        println!("Carte {} : {:?}", i + 1, deck[i]);
       //afficher 
    }
    return deck;
}
//melanger un deck
pub fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    println!("melanger le deck");
}

//piocher une carte du deck
pub fn draw_card()
{
    println!("piocher une carte du deck");
}