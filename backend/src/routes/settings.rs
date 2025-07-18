<<<<<<< HEAD
use actix_web::{error, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_web::{get, patch, post};
use serde_derive::Deserialize;

use crate::email::mailer::Mailer;
use crate::{database::actions, DbPool};


#[derive(Deserialize)]
struct ResetRequest {
    email: String,
}


#[post("/account/request-password-reset")]
async fn request_password_reset_token(json: web::Json<ResetRequest>, pool: web::Data<DbPool>, mailer: web::Data<Mailer>) -> actix_web::Result<impl Responder> {
    let email = json.email.clone();
    let token = uuid::Uuid::new_v4().to_string();
    let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(30)).naive_utc();

    let reset_token = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let account = actions::get_full_account_by_email(&mut conn, &email);

        match account {
            Ok(account) => {
                actions::create_password_reset_token(&mut conn, account.id, &token, &expires_at)
            }
            Err(e) => { Err(e) }
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    // TODO Limit new requests to 1 per 5 minutes

    match mailer.send_password_reset_email(&json.email.as_str(), &reset_token.token.as_str()) {
        Ok(_) => {
            Ok(HttpResponse::Ok().body("Password reset request sent to email address"))
        }
        Err(e) => {
            Err(error::ErrorInternalServerError(e))
        }
    }

}


#[derive(Deserialize)]
struct ResetPassword {
    token: String,
    new_password: String,
}


// Error types for password reset
#[derive(Debug)]
enum ResetPasswordError {
    TokenExpired,
    TokenUsed,
    NotFound,
    Database(String),
}

impl std::fmt::Display for ResetPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResetPasswordError::TokenExpired => write!(f, "Token expired"),
            ResetPasswordError::TokenUsed => write!(f, "Token already used"),
            ResetPasswordError::NotFound => write!(f, "Token not found"),
            ResetPasswordError::Database(e) => write!(f, "Database error: {e}"),
        }
    }
}

impl std::error::Error for ResetPasswordError {}


#[post("/account/reset-password")]
async fn reset_password(json: web::Json<ResetPassword>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        let reset_token = actions::get_password_reset_token(&mut conn, &json.token)
            .map_err(|_| ResetPasswordError::NotFound)?;
        
        if reset_token.expires_at < chrono::Utc::now().naive_utc() {
            return Err(ResetPasswordError::TokenExpired);
        }

        if reset_token.used {
            return Err(ResetPasswordError::TokenUsed);
        }

        // TODO hash password
        actions::reset_password(&mut conn, reset_token, &json.new_password)
            .map_err(|e| ResetPasswordError::Database(e.to_string()))?;

        Ok(())
    })
    .await?;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Ok().body("Password reset successful"))
        }
        Err(ResetPasswordError::TokenExpired) => {
            Err(error::ErrorBadRequest("Token expired"))
        }
        Err(ResetPasswordError::TokenUsed) => {
            Err(error::ErrorBadRequest("Token already used"))
        }
        Err(ResetPasswordError::NotFound) => {
            Err(error::ErrorNotFound("Token not found"))
        }
        Err(e) => {
            Err(error::ErrorInternalServerError(format!("Password reset failed: {e}")))
        }
    }
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(request_password_reset_token)
        .service(reset_password);
}
=======
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
>>>>>>> test_unit
