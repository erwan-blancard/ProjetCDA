use actix_web::{get, patch, post, HttpMessage, HttpRequest};
use actix_web::{error, web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use database::schema::accounts;
use diesel::dsl::insert_into;
use diesel::query_dsl::methods::FilterDsl;
use diesel::query_dsl::methods::SelectDsl;
use diesel::{ExpressionMethods, Insertable, QueryDsl};
use diesel::PgConnection;
use diesel::r2d2;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;

use serde_derive::Deserialize;

mod auth;

// declare database as module
mod database {
    pub mod models;
    pub mod schema;
    pub mod actions;
}

use database::models::*;
use database::actions::{self, NewAccount, AccountLogin};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;


#[get("/account/friends")]
async fn get_friends_for_account(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let requests = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::list_friends_for_account(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(requests))
}

#[get("/account/requests")]
async fn get_friend_requests_for_account(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let requests = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::list_friend_requests_for_account(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(requests))
}

#[get("/account/requests/{username}")]
async fn get_friend_request_by_username(req: HttpRequest, pool: web::Data<DbPool>, path: web::Path<(String,)>) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    let (username,) = path.into_inner();

    let requests = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_friend_request_of_account_by_username(&mut conn, account_id, &username)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(requests))
}

#[derive(Deserialize)]
struct NewFriendRequestJSON {
    username: String,
}

#[post("/account/requests")]
async fn send_friend_request(req: HttpRequest, pool: web::Data<DbPool>, json: web::Json<NewFriendRequestJSON>) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let requests = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::send_friend_request(&mut conn, account_id, &json.username)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(requests))
}

#[derive(Deserialize)]
struct FriendRequestResponseJSON {
    accepted: bool
}


#[patch("/account/requests/{username}")]
async fn change_friend_request_status(req: HttpRequest, pool: web::Data<DbPool>, path: web::Path<(String,)>, json: web::Json<FriendRequestResponseJSON>) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    let (username,) = path.into_inner();

    let request = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::change_friend_request_status(&mut conn, account_id, &username, json.accepted)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(request))
}


#[get("/account/{account_id}/stats")]
async fn get_other_account_stats(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let (account_id,) = path.into_inner();

    let stats = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_stats(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(stats))
}


#[get("/account/stats")]
async fn get_my_account_stats(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let stats = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_stats(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(stats))
}

#[get("/account/{account_id}")]
async fn get_other_account(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let (account_id,) = path.into_inner();

    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(account))
}

#[get("/account")]
async fn get_my_account(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // get account id based on JWT (put in extensions by JwtMiddleware)
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(account))
}

#[post("/register")]
async fn create_account(pool: web::Data<DbPool>, json: web::Json<NewAccount>) -> actix_web::Result<impl Responder> {
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


#[post("/login")]
async fn login(pool: web::Data<DbPool>, json: web::Json<AccountLogin>) -> actix_web::Result<impl Responder> {
    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_for_login(&mut conn, &json.username, &json.password)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let token = auth::create_jwt(account.id);

    Ok(HttpResponse::Ok().json(token))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set !");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect(format!("Unable to connect to database with URL \"{}\" !", database_url).as_str());

    println!("Connected to database!");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
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