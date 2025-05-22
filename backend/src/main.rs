use actix_web::cookie::{Cookie, SameSite};
use actix_web::{get, patch, post, HttpMessage, HttpRequest};
use actix_web::{error, web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use actix_files::Files;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use diesel::dsl::insert_into;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, SelectableHelper};
use serde_derive::Deserialize;

mod auth;

// modules Diesel
mod database {
    pub mod models;
    pub mod schema;
    pub mod actions;
}

use database::models::*;
use database::actions::{self, NewAccount, AccountLogin};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/account/friends")]
async fn get_friends_for_account(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = *req.extensions().get::<i32>().unwrap();
    let friends = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::list_friends_for_account(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(friends))
}

#[get("/account/requests")]
async fn get_friend_requests_for_account(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = *req.extensions().get::<i32>().unwrap();
    let reqs = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::list_friend_requests_for_account(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(reqs))
}

#[get("/account/requests/{username}")]
async fn get_friend_request_by_username(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = *req.extensions().get::<i32>().unwrap();
    let username = path.into_inner().0;
    let req_item = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::get_friend_request_of_account_by_username(&mut conn, account_id, &username)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(req_item))
}

#[derive(Deserialize)]
struct NewFriendRequestJSON {
    username: String,
}

#[post("/account/requests")]
async fn send_friend_request(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    json: web::Json<NewFriendRequestJSON>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = *req.extensions().get::<i32>().unwrap();
    let sent = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::send_friend_request(&mut conn, account_id, &json.username)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(sent))
}

#[derive(Deserialize)]
struct FriendRequestResponseJSON {
    accepted: bool,
}

#[patch("/account/requests/{username}")]
async fn change_friend_request_status(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    json: web::Json<FriendRequestResponseJSON>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = *req.extensions().get::<i32>().unwrap();
    let username = path.into_inner().0;
    let updated = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::change_friend_request_status(&mut conn, account_id, &username, json.accepted)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(updated))
}

#[get("/account/{account_id}/stats")]
async fn get_other_account_stats(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
) -> actix_web::Result<impl Responder> {
    let account_id = path.into_inner().0;
    let stats = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::get_account_stats(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(stats))
}

#[get("/account/stats")]
async fn get_my_account_stats(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = *req.extensions().get::<i32>().unwrap();
    let stats = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::get_account_stats(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(stats))
}

#[get("/account/{account_id}")]
async fn get_other_account(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
) -> actix_web::Result<impl Responder> {
    let account_id = path.into_inner().0;
    let acct = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::get_account(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(acct))
}

#[get("/account")]
async fn get_my_account(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = *req.extensions().get::<i32>().unwrap();
    let acct = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::get_account(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(acct))
}

#[post("/register")]
async fn create_account(
    pool: web::Data<DbPool>,
    json: web::Json<NewAccount>,
) -> actix_web::Result<impl Responder> {
    let acct = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::create_account(&mut conn, &json.username, &json.email, &json.password)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(acct))
}

#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    json: web::Json<AccountLogin>,
) -> actix_web::Result<impl Responder> {
    let acct = web::block(move || {
        let mut conn = pool.get().unwrap();
        actions::get_account_for_login(&mut conn, &json.username, &json.password)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    let token = auth::create_jwt(acct.id);

    let cookie = Cookie::build("token", token.clone())
        .secure(true)
        .same_site(SameSite::None)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(token))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1) Logger (pour Logger::default())
    env_logger::init();

    // 2) Pool Diesel
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set !");
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool");

    println!("Connected to database!");

    HttpServer::new(move || {
        // CORS _avec_ cookies
        let cors = Cors::permissive()
            .supports_credentials();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            // 3) pages statiques dans ./static/
            .service(Files::new("/", "./static").index_file("login.html"))
            .app_data(web::Data::new(pool.clone()))
            .wrap(auth::JwtMiddleware)
            // auth
            .service(login)
            .service(create_account)
            // accounts
            .service(get_my_account)
            .service(get_other_account)
            // stats
            .service(get_my_account_stats)
            .service(get_other_account_stats)
            // friends
            .service(get_friends_for_account)
            .service(get_friend_requests_for_account)
            .service(get_friend_request_by_username)
            .service(send_friend_request)
            .service(change_friend_request_status)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
