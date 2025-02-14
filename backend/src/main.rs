use actix_web::{get, post, patch};
use actix_web::{error, web, App, HttpServer, Responder, HttpResponse};
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

// declare database as module
mod database {
    pub mod models;
    pub mod schema;
    pub mod actions;
}

use database::models::*;
use database::actions::{self, NewAccount, FriendRequest};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;


#[get("/accounts/{account_id}/friends")]
async fn get_friends_for_account(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let (account_id,) = path.into_inner();

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

#[get("/accounts/{account_id}/requests")]
async fn get_friend_requests_for_account(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let (account_id,) = path.into_inner();

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

#[get("/accounts/{account_id}/requests/{username}")]
async fn get_friend_request_by_username(pool: web::Data<DbPool>, path: web::Path<(i32,String)>) -> actix_web::Result<impl Responder> {
    let (account_id, username) = path.into_inner();

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

#[post("/accounts/{account_id}/requests")]
async fn send_friend_request(pool: web::Data<DbPool>, path: web::Path<(i32,)>, json: web::Json<NewFriendRequestJSON>) -> actix_web::Result<impl Responder> {
    let (account_id,) = path.into_inner();

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


#[patch("/accounts/{account_id}/requests/{username}")]
async fn change_friend_request_status(pool: web::Data<DbPool>, path: web::Path<(i32, String)>, json: web::Json<FriendRequestResponseJSON>) -> actix_web::Result<impl Responder> {
    let (account_id, username) = path.into_inner();

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


#[get("/accounts/{account_id}/stats")]
async fn get_account_stats(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
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

#[get("/accounts/{account_id}")]
async fn get_account(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
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

#[post("/accounts")]
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
            .app_data(web::Data::new(pool.clone()))
            // accounts
            .service(get_account)
            .service(create_account)
            // stats
            .service(get_account_stats)
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