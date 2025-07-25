use actix_web::{delete, error, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::{get, patch, post};
use serde_derive::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::routes::game::Lobbies;
use crate::routes::sse::Broadcaster;
use crate::{database::actions, DbPool};
use crate::database::actions::FriendWithLobbyStatus;
use crate::database::models::Friend;

#[derive(Deserialize, ToSchema)]
pub struct NewFriendRequestJSON {
    /// Username of the user to send a friend request to
    pub username: String,
}

#[derive(Deserialize, ToSchema)]
pub struct FriendRequestResponseJSON {
    /// Whether the request is accepted (true) or declined (false)
    pub accepted: bool,
}

#[utoipa::path(
    get,
    path = "/account/friends",
    responses(
        (status = 200, description = "List of friends with lobby status", body = [FriendWithLobbyStatus]),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Friends"
)]
#[get("/account/friends")]
async fn get_friends_for_account(req: HttpRequest, pool: web::Data<DbPool>, lobbies: web::Data<Lobbies>) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let lobbies = lobbies.clone();
    let requests = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        // actions::list_friends_for_account(&mut conn, account_id)
        actions::list_friends_with_status_for_account(&mut conn, account_id, &lobbies)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(requests))
}

#[utoipa::path(
    get,
    path = "/account/requests",
    responses(
        (status = 200, description = "List of friend requests", body = [Friend]),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Friends"
)]
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

#[utoipa::path(
    post,
    path = "/account/requests",
    request_body = NewFriendRequestJSON,
    responses(
        (status = 201, description = "Friend request sent", body = Friend),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Friends"
)]
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

#[utoipa::path(
    patch,
    path = "/account/requests/{username}",
    params(
        ("username" = String, Path, description = "Username of the friend request sender/receiver")
    ),
    request_body = FriendRequestResponseJSON,
    responses(
        (status = 200, description = "Friend request status changed", body = Friend),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Friends"
)]
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

#[utoipa::path(
    delete,
    path = "/account/friends/{username}",
    params(
        ("username" = String, Path, description = "Username of the friend to delete")
    ),
    responses(
        (status = 200, description = "Friendship deleted", body = (i32, String, String)),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Friends"
)]
#[delete("/account/friends/{username}")]
async fn delete_friendship(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    broadcaster: web::Data<Broadcaster>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let (username,) = path.into_inner();

    let (id, acc1, acc2) = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        actions::delete_friendship(&mut conn, account_id, &username)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    broadcaster.notify_friendship_deleted(id, acc1, acc2).await;

    Ok(HttpResponse::Ok().json((id, acc1, acc2)))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_friends_for_account)
        .service(get_friend_requests_for_account)
        .service(send_friend_request)
        .service(change_friend_request_status)
        .service(delete_friendship);
}