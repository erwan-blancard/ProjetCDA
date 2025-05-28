mod start;
mod card;
mod deck;
mod player;
mod tuto_mode;
mod play_info;





fn main() {
    println!("Bienvenue dans Randomi GO !");
    let nom = start::get_name();
    println!("Bonjour, {} !", nom);
    // start::run();
    start::select_mode();
    
}


