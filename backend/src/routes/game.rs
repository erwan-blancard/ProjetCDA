use std::collections::HashSet;

use actix_web::error::{ErrorConflict, ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::{get, post, delete, patch};
use nanoid::nanoid;
use polodb_core::bson::{doc, Document};
use polodb_core::{Collection, CollectionT};
use tokio::spawn;
use tokio::sync::oneshot;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::routes::sse::Broadcaster;
use crate::server::dto::{GameSessionInfo, responses::PlayerProfile};
use crate::server::server::GameServer;
use crate::backend_db::BackendDb;
use crate::{GameHandlers, GameId};
use crate::{database::actions, DbPool};


pub const LOBBY_ID_LEN: usize = 7;
pub const LOBBY_ID_CHARS: [char; 35] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];


pub type LobbyId = String;


pub fn generate_lobby_id(existing_lobbies: &Lobbies) -> Result<String, String> {
    let mut i: usize = 0;
    // attempt to generate new random id
    // in case it was already generated, try again until limit is reached
    // this is very unlikely but possible
    while i < 10 {
        let id = nanoid!(LOBBY_ID_LEN, &LOBBY_ID_CHARS);

        let existing = existing_lobbies.find_one(doc! {
            "id": &id
        }).map_err(|_| "Could not check existing lobbies")?;

        if existing.is_none() {
            return Ok(id);
        }

        i += 1;
    }

    Err("Unable to generate new lobby id !".to_string())
}


/// Simplified struct for Lobby
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct LobbyInfo {
    pub id: String,
    pub users: HashSet<i32>,
    pub users_ready: HashSet<i32>,
    pub ingame: bool
}


#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct Lobby {
    pub id: String,
    pub users: HashSet<i32>,
    pub users_ready: HashSet<i32>,
    /// if unlisted the lobby is not returned by /lobby/list route
    pub unlisted: bool,
    #[schema(value_type = Option<String>)]
    pub game_id: Option<GameId>
}

impl Lobby {
    pub fn new(id: String, unlisted: bool) -> Self {
        Self { id, users: HashSet::new(), users_ready: HashSet::new(), unlisted, game_id: None }
    }

    pub fn all_users_ready(&self) -> bool {
        self.users.iter()
            .map(|id| self.users_ready.get(id).is_some())
            .all(|is_some| is_some)
    }

    pub fn info(&self) -> LobbyInfo {
        LobbyInfo {
            id: self.id.clone(),
            users: self.users.clone(),
            users_ready: self.users_ready.clone(),
            ingame: self.game_id.is_some()
        }
    }
}

pub type Lobbies = Collection<Lobby>;


const LOBBY_PAGE_SIZE: usize = 20;


#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateLobbyInfo {
    #[serde(default)]
    pub unlisted: bool,
}


async fn create_game_session(
    lobby: &mut Lobby,
    game_handlers: web::Data<GameHandlers>,
    pool: web::Data<DbPool>,
    backend_db: web::Data<BackendDb>,
) -> Result<GameId, Error> {
    let user_ids: Vec<i32> = lobby.users
        .iter().cloned().collect();

    let players = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_accounts_by_id(&mut conn, &user_ids)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    let players: Vec<PlayerProfile> = players.iter()
        .map(|acc| PlayerProfile { id: acc.id, name: acc.username.clone() })
        .collect();

    let mut game_handlers = game_handlers.lock().unwrap();

    // generate game id and update lobby
    let game_id: GameId = Uuid::new_v4();

    lobby.game_id = Some(game_id);
    backend_db.lobbies_collection()
        .update_one(doc! {
                "id": &lobby.id
            }, doc! {
                "$set": {
                    "game_id": game_id.to_string()
                }
            }).map_err(|e| ErrorInternalServerError(e.to_string()))?;

    // create server process

    let (ready_tx, ready_rx) = oneshot::channel();

    let (game_server, handle) = GameServer::new(players, game_id, backend_db.get_ref().clone(), ready_tx);
    let proccess = spawn(game_server.run());
    ready_rx.await.map_err(ErrorInternalServerError)?;    // wait for ready signal

    game_handlers.insert(game_id, (proccess, handle));

    Ok(game_id)
}


#[utoipa::path(
    post,
    path = "/lobby/create",
    request_body = CreateLobbyInfo,
    responses(
        (status = 201, description = "Lobby created and joined", body = Lobby),
        (status = 409, description = "User already in a lobby"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Lobby"
)]
#[post("/lobby/create")]
/// create and join lobby
async fn create_lobby(
    req: HttpRequest,
    json: web::Json<CreateLobbyInfo>,
    backend_db: web::Data<BackendDb>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    let lobbies = backend_db.lobbies_collection();
    
    if backend_db.get_lobby_for_user(account_id).is_some() {
        return Err(ErrorConflict("User is already in a lobby !"));
    }

    let lobby_id = generate_lobby_id(&lobbies)
        .map_err(ErrorInternalServerError)?;

    let mut lobby = Lobby::new(lobby_id.clone(), json.unlisted);
    lobby.users.insert(account_id);

    lobbies.insert_one(&lobby)
        .map_err(|e| ErrorInternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Created().json(lobby))

}


#[utoipa::path(
    get,
    path = "/lobby/current",
    responses(
        (status = 302, description = "Current lobby found", body = Lobby),
        (status = 404, description = "User is not in a lobby")
    ),
    security(("jwt" = [])),
    tag = "Lobby"
)]
#[get("/lobby/current")]
async fn get_current_lobby(
    req: HttpRequest,
    backend_db: web::Data<BackendDb>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    if let Some(lobby) = backend_db.get_lobby_for_user(account_id) {
        Ok(HttpResponse::Found().json(lobby))
    } else {
        Err(ErrorNotFound("User is not in a lobby !"))
    }
}


#[derive(Debug, Serialize, ToSchema)]
pub struct LobbyPageList {
    pub entries: Vec<LobbyInfo>,
    pub page: usize,
    pub page_count: usize,
}

#[utoipa::path(
    get,
    path = "/lobby/list/{page}",
    params(("page" = usize, Path, description = "Page number (starts at 0)")),
    responses(
        (status = 200, description = "List of lobbies", body = LobbyPageList)
    ),
    tag = "Lobby"
)]
#[get("/lobby/list/{page}")]
async fn list_lobbies(
    path: web::Path<(usize,)>,
    backend_db: web::Data<BackendDb>,
) -> actix_web::Result<impl Responder> {
    // page count starts at 0
    let (page,) = path.into_inner();

    let page_list = list_lobby_page_list_from_db(&backend_db, page)
        .map_err(|_| ErrorInternalServerError(format!("Could not list lobbies for page {}", page)))?;

    Ok(HttpResponse::Ok().json(page_list))
}


// TODO move to BackendDb struct ?
pub fn list_lobby_page_list_from_db(backend_db: &web::Data<BackendDb>, page: usize) -> Result<LobbyPageList, polodb_core::Error> {
    let lobbies = backend_db.lobbies_collection();

    // count lobbies only if they aren't unlisted
    // output (print): [Document({"count": Int64(...)})]
    let count = lobbies.aggregate(vec![
        doc! {
            "$match": { "unlisted": false },
            "$count": "count",
        }
    ]).run()?
        .collect::<polodb_core::Result<Vec<Document>>>()?
        [0].get_i64("count").unwrap();

    // filter out unlisted lobbies
    let entries = lobbies.find(doc! { "unlisted": false })
        .skip((page * LOBBY_PAGE_SIZE) as u64)
        .limit(LOBBY_PAGE_SIZE as u64)
        .run()?
        .collect::<polodb_core::Result<Vec<Lobby>>>()?;

    let entries = entries.iter()
        .map(Lobby::info)
        .collect();

    let page_count = (count as f64 / LOBBY_PAGE_SIZE as f64).ceil() as usize;

    Ok(LobbyPageList { entries, page, page_count })
}


#[utoipa::path(
    get,
    path = "/lobby/find/{lobby_id}",
    params(("lobby_id" = String, Path, description = "Lobby ID")),
    responses(
        (status = 302, description = "Lobby found", body = LobbyInfo),
        (status = 404, description = "Lobby not found")
    ),
    tag = "Lobby"
)]
#[get("/lobby/find/{lobby_id}")]
async fn get_lobby_info(
    path: web::Path<(LobbyId,)>,
    backend_db: web::Data<BackendDb>,
) -> actix_web::Result<impl Responder> {
    let (lobby_id,) = path.into_inner();
    // ID should only contain uppercase letters
    let lobby_id = lobby_id.to_uppercase();

    let lobbies = backend_db.lobbies_collection();

    if let Some(lobby) = lobbies.find_one(doc! { "id": &lobby_id })
            .map_err(|e| ErrorInternalServerError(e.to_string()))? {
        Ok(HttpResponse::Found().json(lobby.info()))
    } else {
        Err(ErrorNotFound(format!("No lobby with id {}", lobby_id)))
    }
}


#[derive(Debug, Deserialize, ToSchema)]
pub struct LobbyJoinInfo {
    pub lobby_id: LobbyId,
}

#[utoipa::path(
    post,
    path = "/lobby/join",
    request_body = LobbyJoinInfo,
    responses(
        (status = 200, description = "Joined lobby", body = Lobby),
        (status = 404, description = "Lobby not found"),
        (status = 409, description = "User already in a lobby"),
        (status = 400, description = "Lobby is full")
    ),
    security(("jwt" = [])),
    tag = "Lobby"
)]
#[post("/lobby/join")]
async fn join_lobby(
    req: HttpRequest,
    json: web::Json<LobbyJoinInfo>,
    backend_db: web::Data<BackendDb>,
    brodcaster: web::Data<Broadcaster>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    // return immediately if wrong len
    if json.lobby_id.len() != LOBBY_ID_LEN {
        return Err(ErrorNotFound("Lobby doesn't exist !"));
    }

    // ID should only contain uppercase letters
    let lobby_id: LobbyId = json.lobby_id.to_uppercase();

    // handles errors
    let lobby = backend_db.join_lobby(&lobby_id, account_id)?;

    brodcaster.notify_lobby_user_list_update(&lobby, account_id).await;

    Ok(HttpResponse::Ok().json(lobby))
}


#[derive(Debug, Deserialize, ToSchema)]
pub struct LobbyReadyInfo {
    pub ready: bool,
}

#[utoipa::path(
    patch,
    path = "/lobby/current/ready",
    request_body = LobbyReadyInfo,
    responses(
        (status = 200, description = "Ready status updated"),
        (status = 404, description = "User is not in a lobby"),
        (status = 409, description = "All users are already ready"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Lobby"
)]
#[patch("/lobby/current/ready")]
async fn lobby_set_ready(
    req: HttpRequest,
    json: web::Json<LobbyReadyInfo>,
    backend_db: web::Data<BackendDb>,
    broadcaster: web::Data<Broadcaster>,
    pool: web::Data<DbPool>,
    game_handlers: web::Data<GameHandlers>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    let ready = json.ready;

    // handles errors
    let mut lobby = backend_db.update_user_ready_state(account_id, ready)?;

    if lobby.all_users_ready() && lobby.users.len() > 1 {
        // create game
        create_game_session(&mut lobby, game_handlers, pool, backend_db).await?;

        broadcaster.notify_lobby_user_ready(&lobby, account_id, ready).await;
        broadcaster.notify_lobby_game_started(&lobby).await;
    } else {
        broadcaster.notify_lobby_user_ready(&lobby, account_id, ready).await;
    }

    Ok(HttpResponse::Ok().finish())

}


#[utoipa::path(
    post,
    path = "/lobby/current/leave",
    responses(
        (status = 200, description = "Left lobby"),
        (status = 404, description = "User is not in a lobby"),
        (status = 409, description = "All users are already ready"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = [])),
    tag = "Lobby"
)]
#[post("/lobby/current/leave")]
async fn leave_current_lobby(
    req: HttpRequest,
    backend_db: web::Data<BackendDb>,
    broadcaster: web::Data<Broadcaster>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    // handles errors
    let lobby = backend_db.leave_lobby(account_id)?;

    if !lobby.users.is_empty() {
        broadcaster.notify_lobby_user_list_update(&lobby, account_id).await;
    }

    Ok(HttpResponse::Ok().finish())
}


#[utoipa::path(
    get,
    path = "/game/find/{game_id}",
    params(("game_id" = String, Path, description = "Game ID (UUID)")),
    responses(
        (status = 200, description = "Game session info", body = GameSessionInfo),
        (status = 404, description = "Session is closed or invalid ID")
    ),
    tag = "Game"
)]
#[get("/game/find/{game_id}")]
async fn get_game_session_info(
    path: web::Path<(GameId,)>,
    game_handlers: web::Data<GameHandlers>
) -> actix_web::Result<impl Responder> {
    let (game_id,) = path.into_inner();

    let game_handlers = game_handlers.lock().unwrap();
    match game_handlers.get(&game_id) {
        Some((_, handler)) => {
            if !handler.is_closed() {
                Ok(HttpResponse::Ok().json(GameSessionInfo { game_id, players: handler.get_session_info().await } ))
            } else {
                Err(ErrorNotFound("Session is Closed"))
            }
        },
        None => Err(ErrorNotFound("Invalid Game Id"))
    }
}


#[utoipa::path(
    get,
    path = "/game/current",
    responses(
        (status = 302, description = "Current game session found", body = GameSessionInfo),
        (status = 404, description = "No current game")
    ),
    security(("jwt" = [])),
    tag = "Game"
)]
#[get("/game/current")]
async fn get_current_game_session_info(
    req: HttpRequest,
    game_handlers: web::Data<GameHandlers>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    

    log::info!("ROUTE /game/current");
    let game_handlers = game_handlers.lock().unwrap();
    log::info!("ROUTE /game/current -> lock acquired");

    // convert to tokio stream to be able to use async filters
    // let handlers = tokio_stream::iter(game_handlers.values());
    
    let mut info: Option<GameSessionInfo> = None;

    for (game_id, (_, handler)) in game_handlers.iter() {
        log::info!("ROUTE /game/current -> checking handler (closed: {})", handler.is_closed());
        if !handler.is_closed() {
            log::info!("ROUTE /game/current -> handler is not closed");
            let players = handler.get_session_info().await;
            log::info!("ROUTE /game/current -> players gathered");

            if players.iter().any(|prf| prf.id == account_id) {
                info = Some(GameSessionInfo { game_id: *game_id, players });
                break;
            }
        }
    }

    log::info!("ROUTE /game/current -> after info gathering");

    if let Some(info) = info {
        for prf in info.players.iter() {
            println!("Profile: {} {}", prf.id, prf.name);
        }
        Ok(HttpResponse::Found().json(info))
    } else {
        Ok(HttpResponse::NotFound().body("No current game"))
    }

}


#[utoipa::path(
    get,
    path = "/game/list",
    responses(
        (status = 200, description = "List of active game sessions", body = [GameSessionInfo])
    ),
    tag = "Game"
)]
#[get("/game/list")]
async fn list_game_sessions(game_handlers: web::Data<GameHandlers>) -> actix_web::Result<impl Responder> {
    let game_handlers = game_handlers.lock().unwrap();

    // let game_ids: Vec<GameId> = game_handlers.keys()
    //     .cloned()
    //     .collect();

    // for game_id in game_ids.iter() {
    //     println!("Game Id: {}", game_id.to_string());
    // }

    // Ok(HttpResponse::Ok().json(game_ids))

    let mut sessions: Vec<GameSessionInfo> = Vec::new();

    for (game_id, (_, handler)) in game_handlers.iter() {
        if !handler.is_closed() {
            sessions.push(GameSessionInfo { game_id: *game_id, players: handler.get_session_info().await });
        }
    }

    Ok(HttpResponse::Ok().json(sessions))
}


/// TODO add admin role for account and only allow admins for this endpoint
/// + verify logic
#[delete("/game/kill/{game_id}")]
async fn kill_session(
    path: web::Path<(GameId,)>,
    game_handlers: web::Data<GameHandlers>,
    backend_db: web::Data<BackendDb>
) -> actix_web::Result<impl Responder> {
    let (game_id,) = path.into_inner();
    let mut game_handlers = game_handlers.lock().unwrap();

    if let Some((process, handler)) = game_handlers.get(&game_id) {
        if !handler.is_closed() {
            handler.kill_server().await;
            process.abort();

            // Reset ready status in the associated lobby
            backend_db.reset_users_ready_on_game_end(&game_id)?;

            // remove the handler from the map
            game_handlers.remove(&game_id);

            Ok(HttpResponse::Ok().finish())
        } else {
            Err(ErrorNotFound("Session is Closed"))
        }
    } else {
        Err(ErrorNotFound("Game not found"))
    }
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_lobby)
        .service(list_lobbies)
        .service(lobby_set_ready)
        .service(get_current_lobby)
        .service(get_lobby_info)
        .service(join_lobby)
        .service(leave_current_lobby)

        .service(get_game_session_info)
        .service(get_current_game_session_info)
        .service(list_game_sessions);
        // .service(kill_session);
}