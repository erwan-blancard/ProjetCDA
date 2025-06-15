use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use actix_web::error::{ErrorBadRequest, ErrorConflict, ErrorForbidden, ErrorInternalServerError, ErrorNotFound};
use actix_web::{delete, error, patch, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::{get, post};
use tokio::spawn;
use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};

use crate::dto::GameSessionInfo;
use crate::server::dto::responses::PlayerProfile;
use crate::server::game::game::MAX_PLAYERS;
use crate::server::server::GameServer;
use crate::utils::limited_string::LimitedString;
use crate::{GameHandlers, GameId};
use crate::{database::actions, DbPool};


pub type LobbyId = Uuid;
pub type LobbyPassword = LimitedString<9>;


/// Filtered struct for Lobby (password is filtered)
#[derive(Debug, Serialize, Clone)]
pub struct LobbyInfo {
    pub users: HashSet<i32>,
    pub users_ready: HashSet<i32>,
    pub password: bool,
    pub ingame: bool
}


#[derive(Debug, Serialize, Clone)]
pub struct Lobby {
    pub users: HashSet<i32>,
    pub users_ready: HashSet<i32>,
    pub password: Option<LobbyPassword>,
    pub game_id: Option<GameId>
}

impl Lobby {
    pub fn new(password: Option<LobbyPassword>) -> Self {
        Self { users: HashSet::new(), users_ready: HashSet::new(), password, game_id: None }
    }

    pub fn is_private(&self) -> bool {
        self.password.is_some()
    }

    pub fn all_users_ready(&self) -> bool {
        self.users.iter()
            .map(|id| self.users_ready.get(id).is_some())
            .any(|is_some| is_some)
    }

    pub fn info(&self) -> LobbyInfo {
        LobbyInfo {
            users: self.users.clone(),
            users_ready: self.users_ready.clone(),
            password: self.password.is_some(),
            ingame: self.game_id.is_some()
        }
    }
}

pub type Lobbies = Arc<Mutex<HashMap<LobbyId, Lobby>>>;


const LOBBY_PAGE_SIZE: usize = 20;


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateLobbyInfo {
    pub password: Option<LobbyPassword>,
}


fn get_lobby_id_for_user(account_id: i32, lobbies: &HashMap<LobbyId, Lobby>) -> Option<LobbyId> {
    for (lobby_id, lobby) in lobbies.iter() {
        if lobby.users.get(&account_id).is_some() {
            return Some(lobby_id.clone());
        }
    }

    None
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

    let mut password: Option<LobbyPassword> = None;
    // set password if not None and len > 0
    if let Some(pswd) = &json.password {
        if pswd.len() > 0 {
            password = Some(pswd.clone());
        }
    }

    let mut lobby = Lobby::new(password);
    lobby.users.insert(account_id);
    let lobby_id: LobbyId = Uuid::new_v4();

    lobbies.insert(lobby_id, lobby.clone());
    
    Ok(HttpResponse::Created().json(lobby))

    // if !json.players.iter().any(|id| *id == account_id) {
    //     return Err(ErrorBadRequest("The list of players must contain the sender's id !"));
    // }
    
    // // FIXME check if users are not already in a game
    // if json.players.len() > 1 && json.players.len() <= MAX_PLAYERS {

    //     let players = web::block(move || {
    //         let mut conn = pool.get().expect("couldn't get db connection from pool");

    //         actions::get_accounts_by_id(&mut conn, &json.players)
    //     })
    //     .await?
    //     .map_err(error::ErrorInternalServerError)?;

    //     let players: Vec<PlayerProfile> = players.iter()
    //         .map(|acc| PlayerProfile { id: acc.id, name: acc.username.clone() })
    //         .collect();

    //     let mut game_handlers = game_handlers.lock().unwrap();

    //     // create server proccess
    //     let (game_server, handle) = GameServer::new(players);
    //     let proccess = spawn(game_server.run());
        
    //     let game_id: GameId = Uuid::new_v4();
    //     game_handlers.insert(game_id, (proccess, handle));

    //     // return the address of the WebSocket
    //     // match api_url.join(format!("/ws/{}", game_id).as_str()) {
    //     //     Ok(ws_url) => {
    //     //         Ok(HttpResponse::Created().json(ws_url.to_string()))
    //     //     },
    //     //     Err(_) => { Ok(HttpResponse::Created().json(game_id.to_string())) }
    //     // }

    //     Ok(HttpResponse::Created().json(game_id.to_string()))

    // } else {
    //     Err(ErrorBadRequest(format!("Number of players must be between 1 and {}", MAX_PLAYERS)))
    // }
    
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
struct LobbyEntry {
    pub lobby_id: LobbyId,
    #[serde(flatten)]
    pub lobby_info: LobbyInfo
}

#[derive(Debug, Serialize)]
struct LobbyPageList {
    pub entries: Vec<LobbyEntry>,
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

    let entries: Vec<LobbyEntry> = lobbies.iter()
        .skip(page*LOBBY_PAGE_SIZE).take(LOBBY_PAGE_SIZE)
        .map(|(lobby_id, lobby)| LobbyEntry { lobby_id: lobby_id.clone(), lobby_info: lobby.info() })
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
    pub password: Option<LobbyPassword>
}


#[post("/lobby/join")]
async fn join_lobby(
    req: HttpRequest,
    json: web::Json<LobbyJoinInfo>,
    lobbies: web::Data<Lobbies>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    let mut lobbies = lobbies.lock().unwrap();
    
    if get_lobby_id_for_user(account_id, &lobbies).is_some() {
        return Err(ErrorConflict("User is already in a lobby !"));
    }

    if let Some(lobby) = lobbies.get_mut(&json.lobby_id) {
        if lobby.is_private() && json.password.is_none() {
            return Err(ErrorForbidden("Lobby is private !"))
        } else if lobby.is_private() && json.password.is_some() {
            let json_passwd = json.password.clone().unwrap();
            let lobby_passwd = lobby.password.clone().unwrap();

            if json_passwd != lobby_passwd {
                return Err(ErrorForbidden("Wrong Password"));
            }
        }

        // join lobby

        lobby.users.insert(account_id);

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
    lobbies: web::Data<Lobbies>
) -> actix_web::Result<impl Responder> {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    let mut lobbies = lobbies.lock().unwrap();

    let lobby_id = get_lobby_id_for_user(account_id, &lobbies);

    if let Some(lobby_id) = lobby_id {

        let lobby = lobbies.get_mut(&lobby_id).unwrap();

        if lobby.all_users_ready() {
            Err(ErrorConflict("Can't update because all users are ready !"))
        } else if json.ready {
            lobby.users_ready.insert(account_id);
            Ok(HttpResponse::Ok().finish())
        } else {
            lobby.users_ready.remove(&account_id);
            Ok(HttpResponse::Ok().finish())
        }

    } else {
        Err(ErrorNotFound("User is not in a lobby !"))
    }

}


#[post("/lobby/current/leave")]
async fn leave_current_lobby(
    req: HttpRequest,
    lobbies: web::Data<Lobbies>
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
    
    let game_handlers = game_handlers.lock().unwrap();

    // convert to tokio stream to be able to use async filters
    // let handlers = tokio_stream::iter(game_handlers.values());
    
    let mut info: Option<GameSessionInfo> = None;

    for (game_id, (_, handler)) in game_handlers.iter() {
        let players = handler.get_session_info().await;
        
        log::info!("Players: ");
        for prf in players.iter() {
            log::info!("-> {} ({})", prf.name, prf.id);
        }

        if players.iter().any(|prf| prf.id == account_id) {
            info = Some(GameSessionInfo { game_id: *game_id, players });
            break;
        }
    }

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
async fn kill_session(path: web::Path<(GameId,)>, game_handlers: web::Data<GameHandlers>) -> actix_web::Result<impl Responder> {
    let (game_id,) = path.into_inner();
    let game_handlers = game_handlers.lock().unwrap();

    if let Some((process, handler)) = game_handlers.get(&game_id) {
        if !handler.is_closed() {
            handler.kill_server().await;
            process.abort();

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

        .service(get_game_session_info)
        .service(get_current_game_session_info)
        .service(list_game_sessions)
        .service(kill_session)
        ;
}
