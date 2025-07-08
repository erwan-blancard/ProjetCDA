use actix_web::{error, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_web::{get, patch, post};
use serde_derive::Deserialize;

use crate::{database::actions, DbPool};


#[derive(Deserialize)]
struct PasswordResetInfo {
    reset_token: String,
    new_password: String
}


#[post("/account/settings/password_reset")]
async fn reset_password() -> actix_web::Result<impl Responder> {

}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // À compléter selon les besoins réels de settings
}
