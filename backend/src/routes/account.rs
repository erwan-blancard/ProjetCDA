use actix_web::{error, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_web::{get, patch, post};
use serde_derive::Deserialize;

use crate::routes::sse::Broadcaster;
use crate::{database::actions, DbPool};


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
async fn send_friend_request(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    json: web::Json<NewFriendRequestJSON>,
    broadcaster: web::Data<Broadcaster>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let request = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::send_friend_request(&mut conn, account_id, &json.username)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    broadcaster.notify_friend_request_update(&request).await;

    Ok(HttpResponse::Created().json(request))
}

#[derive(Deserialize)]
struct FriendRequestResponseJSON {
    accepted: bool
}


#[patch("/account/requests/{username}")]
async fn change_friend_request_status(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    json: web::Json<FriendRequestResponseJSON>,
    broadcaster: web::Data<Broadcaster>
) -> actix_web::Result<impl Responder> {
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

    broadcaster.notify_friend_request_update(&request).await;

    Ok(HttpResponse::Ok().json(request))
}


#[get("/account/stats/{account_id}")]
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

#[get("/account/profile/{account_id}")]
async fn get_other_account(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let (account_id,) = path.into_inner();

    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_by_id(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(account))
}

#[get("/account/profile")]
async fn get_my_account(req: HttpRequest, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // get account id based on JWT (put in extensions by JwtMiddleware)
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_account_by_id(&mut conn, account_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(account))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_my_account)
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
        ;
}