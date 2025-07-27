use actix_files::NamedFile;
use actix_web::{get, web};


#[get("/cards")]
pub async fn get_cards_collection() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(std::env::var("CARDS_FILE_PATH").expect("CARDS_FILE_PATH not set !"))?)
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cards_collection);
}