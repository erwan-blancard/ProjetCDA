use std::io;

use actix_files::NamedFile;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware, web};
use tokio::{
    task::{spawn, spawn_local},
    try_join,
};

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


async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Handshake and start WebSocket handler with heartbeats.
async fn game_ws(
    req: HttpRequest,
    stream: web::Payload,
    chat_server: web::Data<GameServerHandle>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(handler::game_ws(
        (**chat_server).clone(),
        session,
        msg_stream,
    ));

    Ok(res)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let (chat_server, server_tx) = GameServer::new();

    let chat_server = spawn(chat_server.run());

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_tx.clone()))
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket routes
            .service(web::resource("/ws").route(web::get().to(game_ws)))
            // standard middleware
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8081))?
    .run();

    try_join!(http_server, async move { chat_server.await.unwrap() })?;

    Ok(())
}
