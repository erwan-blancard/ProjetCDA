use std::io;

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware, web};
use tokio::{
    task::{spawn, spawn_local},
    try_join,
};

use uid::IdU64;

mod api;
mod handler;
mod server;
mod dto {
    pub mod actions;
    pub mod responses;
}
pub mod game {
    pub mod engine;
}

pub use self::server::{GameServer, GameServerHandle};

/// Connection ID.
pub type ConnId = u64;

pub type Msg = String;
pub type Token = String;


#[derive(Debug)]
struct Player {
    token: Token,
    name: String,
}

/// Handshake and start WebSocket handler with heartbeats.
async fn game_ws(
    req: HttpRequest,
    stream: web::Payload,
    game_server: web::Data<GameServerHandle>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(handler::game_ws(
        (**game_server).clone(),
        session,
        msg_stream,
    ));

    Ok(res)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let api_url = std::env::var("API_URL").expect("API_URL is not set !");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let (game_server, server_tx) = GameServer::new();

    let game_server = spawn(game_server.run());

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_tx.clone()))
            // websocket routes
            .service(web::resource("/ws").route(web::get().to(game_ws)))
            // standard middleware
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8081))?
    .run();

    try_join!(http_server, async move { game_server.await.unwrap() })?;

    Ok(())
}
