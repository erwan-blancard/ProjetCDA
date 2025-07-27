use actix_files::NamedFile;
use actix_web::{get, web};
use utoipa::ToSchema;

use crate::server::game::cards::card::{CardId, Element, Kind, Stars};


// Schema defined here (stripped down version of crate::server::game::database::CardInfo)
#[derive(ToSchema)]
pub struct CardInfo {
    id: CardId,
    name: String,
    #[schema(value_type = String)]
    element: Element,
    #[schema(value_type = String)]
    stars: Stars,
    #[schema(value_type = String)]
    kind: Kind,
    desc: String,
}


#[utoipa::path(
    get,
    path = "/cards",
    responses(
        (status = 200, description = "Get cards definitions", body = [CardInfo]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Cards"
)]
#[get("/cards")]
pub async fn get_cards_collection() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(std::env::var("CARDS_FILE_PATH").expect("CARDS_FILE_PATH not set !"))?)
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cards_collection);
}