use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use core::time::Duration;

use actix_web::error::ErrorNotFound;
use actix_web::{get, HttpMessage, HttpRequest};
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::middleware::{Logger, NormalizePath};
use actix_cors::Cors;
use diesel::PgConnection;
use diesel::r2d2;
use reqwest::Url;
use server::handler;
use server::server::GameServerHandle;
use tokio::time::Instant;
use tokio::{self, spawn};


// expose modules

mod auth;
mod utils {
    pub mod limited_string;
    pub mod clamp;
}

mod dto;
mod routes {
    pub mod account;
    pub mod auth;
    pub mod game;
    pub mod sse;
}

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
        /// This module contains special cards that don't fit into the normal card system
        pub mod special_cards;
        pub mod database;
        pub mod game;
        pub mod modifiers;
        pub mod player;
        pub mod play_info;
    }
}

// type ApiUrl = Url;
type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

use tokio::task::{spawn_local, JoinHandle};
use uuid::Uuid;

use crate::routes::game::{Lobbies, Lobby, LobbyId};
use crate::routes::sse::Broadcaster;

// use uid::Id as IdT;

// #[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
// struct T(());

// type GameId = IdT<T>;
type GameId = Uuid;

type GameJoinHandle = JoinHandle<Result<(), std::io::Error>>;
type GameHandlers = Arc<Mutex<HashMap<GameId, (GameJoinHandle, GameServerHandle)>>>;

async fn purge_server_handlers_periodic(server_handlers: GameHandlers, period: Duration) {
    let mut interval = tokio::time::interval_at(Instant::now() + period, period);

    loop {
        interval.tick().await;

        {
            let mut handlers = server_handlers.lock().unwrap();

            // keep handlers that are not finished
            handlers.retain(|&_, (child, _)| !child.is_finished());
        }
    }
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

            if handle.is_closed() {
                return Err(ErrorNotFound("Game is over"));
            }
            
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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // let api_url: ApiUrl = ApiUrl::parse(&std::env::var("BACKEND_URL").unwrap_or(String::new())).expect("BACKEND_URL is not valid !");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set !");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect(format!("Unable to connect to database with URL \"{}\" !", database_url).as_str());

    println!("Connected to database!");

    let lobbies: Lobbies = Arc::new(Mutex::new(HashMap::<LobbyId, Lobby>::new()));
    let server_handlers: GameHandlers = Arc::new(Mutex::new(HashMap::<GameId, (GameJoinHandle, GameServerHandle)>::new()));
    let handlers_to_purge = server_handlers.clone();

    // purge server processes periodically
    spawn(async move {
        purge_server_handlers_periodic(handlers_to_purge, Duration::from_secs(120)).await;
    });

    let broadcaster = Broadcaster::create();

    HttpServer::new(move || {
        // FIXME configure
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(lobbies.clone()))
            .app_data(web::Data::new(server_handlers.clone()))
            // .app_data(web::Data::new(api_url.clone()))
            .app_data(web::Data::from(Arc::clone(&broadcaster)))
            .wrap(cors)
            .wrap(auth::JwtMiddleware)
            .wrap(NormalizePath::trim())    // normalizes paths (no trailing "/")

            // auth
            .configure(routes::auth::configure_routes)
            // accounts, stats, friends
            .configure(routes::account::configure_routes)
            // game
            .configure(routes::game::configure_routes)
            // sse
            .configure(routes::sse::configure_routes)

            // ws
            .service(connect_to_ws)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}