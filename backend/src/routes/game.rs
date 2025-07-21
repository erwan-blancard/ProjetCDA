use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use actix_web::error::{ErrorBadRequest, ErrorConflict, ErrorForbidden, ErrorInternalServerError, ErrorNotFound};
use actix_web::{delete, error, patch, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::{get, post};
use nanoid::nanoid;
use tokio::spawn;
use tokio::sync::oneshot;
use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};

use crate::routes::sse::Broadcaster;
use crate::server::dto::{GameSessionInfo, responses::PlayerProfile};
use crate::server::game::game::MAX_PLAYERS;
use crate::server::server::GameServer;
use crate::{GameHandlers, GameId};
use crate::{database::actions, DbPool};


pub const LOBBY_ID_LEN: usize = 7;
pub const LOBBY_ID_CHARS: [char; 35] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];


pub type LobbyId = String;


pub fn generate_lobby_id(existing_lobbies: &LobbiesInner) -> Result<String, String> {
    let mut i: usize = 0;
    // attempt to generate new random id
    // in case it was already generated, try again until limit is reached
    // this is very unlikely but possible
    while i < 10 {
        let id = nanoid!(LOBBY_ID_LEN, &LOBBY_ID_CHARS);

        if existing_lobbies.get(&id).is_none() {
            return Ok(id);
        }

        i += 1;
    }

    Err("Unable to generate new lobby id !".to_string())
}


/// Simplified struct for Lobby
#[derive(Debug, Serialize, Clone)]
pub struct LobbyInfo {
    pub id: String,
    pub users: HashSet<i32>,
    pub users_ready: HashSet<i32>,
    pub ingame: bool
}


#[derive(Debug, Serialize, Clone)]
pub struct Lobby {
    pub id: String,
    pub users: HashSet<i32>,
    pub users_ready: HashSet<i32>,
    /// if unlisted the lobby is not returned by /lobby/list route
    pub unlisted: bool,
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

pub type LobbiesInner = HashMap<LobbyId, Lobby>;
pub type Lobbies = Arc<Mutex<LobbiesInner>>;


const LOBBY_PAGE_SIZE: usize = 20;


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateLobbyInfo {
    #[serde(default)]
    pub unlisted: bool,
}


fn get_lobby_id_for_user(account_id: i32, lobbies: &HashMap<LobbyId, Lobby>) -> Option<LobbyId> {
    println!("acc id: {}", account_id);
    for (lobby_id, lobby) in lobbies.iter() {
        for user_id in lobby.users.iter() {
            println!("id {}", user_id);
        }
        if lobby.users.get(&account_id).is_some() {
            return Some(lobby_id.clone());
        }
    }

    None
}

async fn create_game_session(
    lobby: &mut Lobby,
    game_handlers: web::Data<GameHandlers>,
    pool: web::Data<DbPool>,
    lobbies: web::Data<Lobbies>,
) -> Result<GameId, Error> {
    let user_ids: Vec<i32> = lobby.users
        .iter().cloned().collect();

    let players = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        actions::get_accounts_by_id(&mut conn, &user_ids)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let players: Vec<PlayerProfile> = players.iter()
        .map(|acc| PlayerProfile { id: acc.id, name: acc.username.clone() })
        .collect();

    let mut game_handlers = game_handlers.lock().unwrap();

    // create server process
    let game_id: GameId = Uuid::new_v4();

    // wait for ready signal
    let (ready_tx, ready_rx) = oneshot::channel();

    let (game_server, handle) = GameServer::new(players, game_id, lobbies.get_ref().clone(), ready_tx);
    let proccess = spawn(game_server.run());
    ready_rx.await.map_err(error::ErrorInternalServerError)?;    // wait for ready signal

    game_handlers.insert(game_id, (proccess, handle));

    Ok(game_id)
}


#[post("/lobby/create")]
/// create and join lobby
async fn create_lobby(
    req: HttpRequest,
    json: web::Json<CreateLobbyInfo>,
    lobbies: web::Data<Lobbies>,
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    let mut lobbies = lobbies.lock().unwrap();
    
    if get_lobby_id_for_user(account_id, &lobbies).is_some() {
        return Err(ErrorConflict("User is already in a lobby !"));
    }

    let lobby_id = generate_lobby_id(&lobbies)
        .map_err(error::ErrorInternalServerError)?;

    let mut lobby = Lobby::new(lobby_id.clone(), json.unlisted);
    lobby.users.insert(account_id);

    lobbies.insert(lobby_id, lobby.clone());
    
    Ok(HttpResponse::Created().json(lobby))

}


#[get("/lobby/current")]
async fn get_current_lobby(
    req: HttpRequest,
    lobbies: web::Data<Lobbies>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();

    let lobbies = lobbies.lock().unwrap();

    if let Some(lobby_id) = get_lobby_id_for_user(account_id, &lobbies) {
        Ok(HttpResponse::Found().json(lobbies.get(&lobby_id)))
    } else {
        Err(ErrorNotFound("User is not in a lobby !"))
    }
}


#[derive(Debug, Serialize)]
struct LobbyPageList {
    pub entries: Vec<LobbyInfo>,
    pub page: usize,
    pub page_count: usize,
}


#[get("/lobby/list/{page}")]
async fn list_lobbies(
    path: web::Path<(usize,)>,
    lobbies: web::Data<Lobbies>
) -> actix_web::Result<impl Responder> {
    // page count starts at 0
    let (page,) = path.into_inner();

    let lobbies = lobbies.lock().unwrap();

    let entries: Vec<LobbyInfo> = lobbies.iter()
        .skip(page*LOBBY_PAGE_SIZE)
        .filter_map(|(_, lobby)| if !lobby.unlisted { Some(lobby.info()) } else { None })
        .take(LOBBY_PAGE_SIZE)
        .collect();

    let page_count = (lobbies.len() as f64 / LOBBY_PAGE_SIZE as f64).ceil() as usize;

    Ok(HttpResponse::Ok().json(LobbyPageList { entries, page, page_count }))
}


#[get("/lobby/find/{lobby_id}")]
async fn get_lobby_info(
    path: web::Path<(LobbyId,)>,
    lobbies: web::Data<Lobbies>
) -> actix_web::Result<impl Responder> {
    let (lobby_id,) = path.into_inner();
    // ID should only contain uppercase letters
    let lobby_id = lobby_id.to_uppercase();
    let lobbies = lobbies.lock().unwrap();

    if let Some(lobby) = lobbies.get(&lobby_id) {
        Ok(HttpResponse::Found().json(lobby.info()))
    } else {
        Err(ErrorNotFound(format!("No lobby with id {}", lobby_id)))
    }
}


#[derive(Debug, Deserialize)]
struct LobbyJoinInfo {
    pub lobby_id: LobbyId,
}


#[post("/lobby/join")]
async fn join_lobby(
    req: HttpRequest,
    json: web::Json<LobbyJoinInfo>,
    lobbies: web::Data<Lobbies>,
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
    let lobby_id = json.lobby_id.to_uppercase();

    let mut lobbies = lobbies.lock().unwrap();
    
    if get_lobby_id_for_user(account_id, &lobbies).is_some() {
        return Err(ErrorConflict("User is already in a lobby !"));
    }

    if let Some(lobby) = lobbies.get_mut(&lobby_id) {
        if lobby.users.len() + 1 > MAX_PLAYERS {
            return Err(ErrorBadRequest("Lobby is full !"));
        }

        // join lobby

        lobby.users.insert(account_id);

        brodcaster.notify_lobby_user_list_update(&lobby, account_id).await;

        return Ok(HttpResponse::Ok().json(lobby));
    } else {
        Err(ErrorNotFound("Lobby doesn't exist !"))
    }
}


#[derive(Debug, Deserialize)]
struct LobbyReadyInfo {
    pub ready: bool,
}


// TODO create game server when everyone is ready

#[patch("/lobby/current/ready")]
async fn lobby_set_ready(
    req: HttpRequest,
    json: web::Json<LobbyReadyInfo>,
    lobbies: web::Data<Lobbies>,
    broadcaster: web::Data<Broadcaster>,
    pool: web::Data<DbPool>,
    game_handlers: web::Data<GameHandlers>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    let mut lobbies_map = lobbies.lock().unwrap();

    let lobby_id = get_lobby_id_for_user(account_id, &lobbies_map);

    if let Some(lobby_id) = lobby_id {

        let lobby = lobbies_map.get_mut(&lobby_id).unwrap();

        if lobby.all_users_ready() && lobby.users.len() > 1 {
            return Err(ErrorConflict("Can't update because all users are ready !"));
        }

        if json.ready { lobby.users_ready.insert(account_id); }
        else { lobby.users_ready.remove(&account_id); }

        broadcaster.notify_lobby_user_ready(lobby, account_id, false).await;

        if lobby.all_users_ready() && lobby.users.len() > 1 {
            // create game
            match create_game_session(lobby, game_handlers, pool, lobbies.clone()).await {
                Ok(game_id) => { lobby.game_id = Some(game_id); },
                Err(_) => { return Err(error::ErrorInternalServerError("Could not create game session")); }
            }

            broadcaster.notify_lobby_game_started(lobby).await;
        }

        Ok(HttpResponse::Ok().finish())

    } else {
        Err(ErrorNotFound("User is not in a lobby !"))
    }

}


#[post("/lobby/current/leave")]
async fn leave_current_lobby(
    req: HttpRequest,
    lobbies: web::Data<Lobbies>,
    broadcaster: web::Data<Broadcaster>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    let mut lobbies = lobbies.lock().unwrap();

    let lobby_id = get_lobby_id_for_user(account_id, &lobbies);

    if let Some(lobby_id) = lobby_id {

        let lobby = lobbies.get_mut(&lobby_id).unwrap();

        if lobby.all_users_ready() {
            Err(ErrorConflict("Can't leave because all users are ready !"))
        } else {
            lobby.users.remove(&account_id);
            lobby.users_ready.remove(&account_id);

            if lobby.users.is_empty() {
                // remove lobby
                lobbies.remove(&lobby_id);

            } else {
                broadcaster.notify_lobby_user_list_update(&lobby, account_id).await;
            }

            Ok(HttpResponse::Ok().finish())
        }

    } else {
        Err(ErrorNotFound("User is not in a lobby !"))
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


#[delete("/game/kill/{game_id}")]
async fn kill_session(path: web::Path<(GameId,)>, game_handlers: web::Data<GameHandlers>, lobbies: web::Data<Lobbies>) -> actix_web::Result<impl Responder> {
    let (game_id,) = path.into_inner();
    let mut game_handlers = game_handlers.lock().unwrap();

    if let Some((process, handler)) = game_handlers.get(&game_id) {
        if !handler.is_closed() {
            handler.kill_server().await;
            process.abort();

            // Reset ready status in the associated lobby
            {
                let mut lobbies_map = lobbies.lock().unwrap();
                let maybe_lobby = lobbies_map.iter_mut().find(|(_, lobby)| lobby.game_id == Some(game_id));
                if let Some((_lobby_id, lobby)) = maybe_lobby {
                    lobby.users_ready.clear();
                }
            }

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
        .service(list_game_sessions)
        .service(kill_session);
}
