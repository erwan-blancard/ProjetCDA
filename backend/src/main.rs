use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use core::time::Duration;

use actix_web::cookie::{Cookie, SameSite};
use actix_web::error::{ErrorBadRequest, ErrorNotFound};
use actix_web::http::header::ContentType;
use actix_web::http::Error;
use actix_web::{get, patch, post, HttpMessage, HttpRequest};
use actix_web::{error, web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::{Logger, NormalizePath};
use actix_cors::Cors;
use database::schema::accounts;
use diesel::expression::is_aggregate::No;
use diesel::PgConnection;
use diesel::r2d2;
use futures::StreamExt;
use reqwest::Url;
use serde::Serialize;
use server::dto::responses::PlayerProfile;
use server::game::game::MAX_PLAYERS;
use server::game::player::Player;
use server::handler;
use server::server::{GameServer, GameServerHandle};
use tokio::time::Instant;
use tokio::{self, spawn};

use serde_derive::Deserialize;

// expose modules

mod auth;

mod database {
    pub mod models;
    pub mod schema;
    pub mod actions;
}

mod server {
    pub mod server;
    pub mod handler;
    pub mod dto {
        pub mod actions;
        pub mod responses;
    }
    pub mod game {
        pub mod card;
        pub mod database;
        pub mod game;
        pub mod player;
        pub mod play_info;
    }
}

use database::models::*;
use database::actions::{self, get_account_by_id, AccountLogin, CreateGameInfo, NewAccount};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

use tokio::task::{spawn_local, JoinHandle};
use uuid::Uuid;

// use uid::Id as IdT;

// #[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
// struct T(());

// type GameId = IdT<T>;
type GameId = Uuid;

type GameJoinHandle = JoinHandle<Result<(), std::io::Error>>;
type GameHandlers = Arc<Mutex<HashMap<GameId, (GameJoinHandle, GameServerHandle)>>>;


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

        actions::get_account_by_id(&mut conn, account_id)
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

        actions::get_account_by_id(&mut conn, account_id)
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

    // create a cookie containing the token and send it to the user
    let cookie = Cookie::build("token", token.clone())
        // FIXME cookie config is permissive
        .secure(true)
        .same_site(SameSite::None)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(token))
}


#[post("/game/create")]
async fn create_game_session(
    req: HttpRequest,
    json: web::Json<CreateGameInfo>,
    game_handlers: web::Data<GameHandlers>,
    pool: web::Data<DbPool>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    // FIXME check if users are not already in a game
    if json.players.len() > 1 && json.players.len() <= MAX_PLAYERS {

        let players = web::block(move || {
            let mut conn = pool.get().expect("couldn't get db connection from pool");

            actions::get_accounts_by_id(&mut conn, &json.players)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        let players: Vec<PlayerProfile> = players.iter()
            .map(|acc| PlayerProfile { id: acc.id, name: acc.username.clone() })
            .collect();

        let mut game_handlers = game_handlers.lock().unwrap();
        let (game_server, handle) = GameServer::new(players);
        let proccess = spawn(game_server.run());
        
        let game_id: GameId = Uuid::new_v4();
        game_handlers.insert(game_id, (proccess, handle));

        // return the address of the WebSocket
        let base_url = Url::parse(req.connection_info().host()).unwrap();
        let ws_url = base_url.join(format!("/ws/{}", game_id).as_str()).unwrap();
        Ok(HttpResponse::Created().json(ws_url.to_string()))

    } else {
        Err(ErrorBadRequest(format!("Number of players must be between 1 and {}", MAX_PLAYERS)))
    }
    
}


#[get("/game/find/{game_id}")]
async fn get_game_session_info(
    path: web::Path<(GameId,)>,
    game_handlers: web::Data<GameHandlers>
) -> actix_web::Result<impl Responder> {
    let (game_id,) = path.into_inner();

    let game_handlers = game_handlers.lock().unwrap();
    match game_handlers.get(&game_id) {
        Some((_, handler)) => Ok(HttpResponse::Ok().json(handler.get_session_info().await)),
        None => Err(ErrorNotFound("Invalid Game Id"))
    }
}


#[get("/game/current")]
async fn get_current_game_session_info(
    req: HttpRequest,
    game_handlers: web::Data<GameHandlers>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    let game_handlers = game_handlers.lock().unwrap();

    // convert to tokio stream to be able to use async filters
    // let handlers = tokio_stream::iter(game_handlers.values());
    
    let mut info: Option<Vec<PlayerProfile>> = None;

    for (_, handler) in game_handlers.values() {
        let players = handler.get_session_info().await;

        if players.iter().any(|prf| prf.id == account_id) {
            info = Some(players);
            break;
        }
    }

    if let Some(info) = info {
        for prf in info.iter() {
            println!("Profile: {} {}", prf.id, prf.name);
        }
        Ok(HttpResponse::Found().json(info))
    } else {
        Ok(HttpResponse::NotFound().json(Vec::<PlayerProfile>::new()))
    }

}


#[get("/game/list")]
async fn list_game_sessions(game_handlers: web::Data<GameHandlers>) -> actix_web::Result<impl Responder> {
    let game_handlers = game_handlers.lock().unwrap();

    let game_ids: Vec<GameId> = game_handlers.keys()
        .cloned()
        .collect();

    for game_id in game_ids.iter() {
        println!("Game Id: {}", game_id.to_string());
    }

    Ok(HttpResponse::Ok().json(game_ids))
}


/// Handshake and start WebSocket handler with heartbeats.
#[get("/ws/{game_id}")]
async fn connect_to_ws(
    req: HttpRequest,
    stream: web::Payload,
    game_handlers: web::Data<GameHandlers>,
    path: web::Path<(GameId,)>
) -> Result<HttpResponse, actix_web::Error> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    let (game_id, ) = path.into_inner();
    
    let game_handlers = game_handlers.lock().unwrap();

    match game_handlers.get(&game_id) {
        Some((_, handle)) => {
            
            let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
            
            // spawn websocket handler (and don't await it) so that the response is returned immediately
            spawn_local(handler::game_ws(
                handle.clone(),
                session,
                msg_stream,
                account_id
            ));

            Ok(res)

        },
        None => { Err(ErrorNotFound("Invalid Game Id")) }
    }
}


async fn purge_server_handlers_periodic(server_handlers: GameHandlers, period: Duration) {
    let mut interval = tokio::time::interval_at(Instant::now() + period, period);

    loop {
        interval.tick().await;

        let mut handlers = server_handlers.lock().unwrap();

        // keep handlers that are not finished
        handlers.retain(|&_, (child, _)| !child.is_finished());
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set !");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect(format!("Unable to connect to database with URL \"{}\" !", database_url).as_str());

    println!("Connected to database!");

    let server_handlers: GameHandlers = Arc::new(Mutex::new(HashMap::<GameId, (GameJoinHandle, GameServerHandle)>::new()));
    let handlers_to_purge = server_handlers.clone();

    // purge server processes periodically
    spawn(async move {
        purge_server_handlers_periodic(handlers_to_purge, Duration::from_secs(120)).await;
    });

    HttpServer::new(move || {
        let cors = Cors::permissive();         // FIXME configure

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(server_handlers.clone()))
            .wrap(auth::JwtMiddleware)
            .wrap(NormalizePath::trim())    // normalizes paths (no trailing "/")

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

            // game
            .service(create_game_session)
            .service(get_game_session_info)
            .service(get_current_game_session_info)
            .service(list_game_sessions)

            // ws
            .service(connect_to_ws)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}