mod card;
mod database;
mod game;
mod player;
mod tuto_mode;
mod start;
mod play_info;


fn main() {
    println!("Bienvenue dans Randomi GO !");
    let nom = start::get_name();
    println!("Bonjour, {} !", nom);
    // start::run();
    start::select_mode();
    
}
