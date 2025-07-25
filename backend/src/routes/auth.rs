use actix_web::{cookie::{Cookie, SameSite}, error::{self, ErrorUnauthorized}, post, web, HttpResponse, Responder};

use crate::{auth, database::{actions::{self, AccountLogin, NewAccount}, models::FilteredAccount}, DbPool};


#[utoipa::path(
    post,
    path = "/register",
    request_body = NewAccount,
    responses(
        (status = 201, description = "Account created successfully", body = FilteredAccount),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
#[post("/register")]
async fn register(pool: web::Data<DbPool>, json: web::Json<NewAccount>) -> actix_web::Result<impl Responder> {
    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::create_account(&mut conn, &json.username, &json.email, &json.password)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(account))
}


#[utoipa::path(
    post,
    path = "/login",
    request_body = AccountLogin,
    responses(
        (status = 200, description = "Login successful, returns JWT token as string", body = String),
        (status = 401, description = "Unauthorized or suspended account"),
        (status = 404, description = "Account not found")
    ),
    tag = "Auth"
)]
#[post("/login")]
async fn login(pool: web::Data<DbPool>, json: web::Json<AccountLogin>) -> actix_web::Result<impl Responder> {
    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_for_login(&mut conn, &json.username, &json.password)
    })
    .await?
    .map_err(error::ErrorNotFound)?;

    if account.suspended {
        return Err(ErrorUnauthorized("The account is suspended."));
    }

    let token = auth::create_jwt(account.id);

    // create a cookie containing the token and send it to the user
    let cookie = Cookie::build("token", token.clone())
        // FIXME cookie config is permissive
        .secure(true)
        .same_site(SameSite::None)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(token))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register);
}