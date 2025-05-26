use std::io;
use crate::card::{Card, Element, Nature, Stars};

use crate::tuto_mode;







pub fn get_name() -> String {
    println!("Quel est ton nom ?");

    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Échec de la lecture");

    nom.trim().to_string()
}

pub fn select_mode()
{
    println!("Sélectionnez le mode de jeu :");
    println!("1. Mode solo");
    println!("2. Mode multijoueur");

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Échec de la lecture");

    match choix.trim() {
        "1" =>  call_tuto(),
        "2" => println!("Mode multijoueur sélectionné"),
        _ => println!("Choix invalide, veuillez réessayer."),
    }
}
pub fn call_tuto()
{
    println!("ok mode tuto selectionné");
    tuto_mode::test();
    
}
pub fn run() 
{
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

    println!("Carte créée : {:?}", boule_de_feu);
}

