use std::collections::HashSet;
use std::fmt::Debug;
use std::{fs::File, io::BufReader, path::Path};

use actix_web::error::{ErrorBadRequest, ErrorConflict, ErrorInternalServerError, ErrorNotFound};
use nanoid::nanoid;
use polodb_core::bson::{doc, Document};
use polodb_core::{Collection, CollectionT, Database};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::game::card_info::{CardInfo, CardInfoList};
use crate::server::game::cards::card::Card;
use crate::server::game::game::MAX_PLAYERS;
use crate::GameId;


pub type LobbyId = String;

pub const LOBBY_ID_LEN: usize = 7;
pub const LOBBY_ID_CHARS: [char; 35] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

const LOBBY_PAGE_SIZE: usize = 20;


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


/// Simplified struct for Lobby
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct LobbyInfo {
    pub id: String,
    pub users: HashSet<i32>,
    pub users_ready: HashSet<i32>,
    pub ingame: bool
}


#[derive(Debug, Serialize, ToSchema)]
pub struct LobbyPageList {
    pub entries: Vec<LobbyInfo>,
    pub page: usize,
    pub page_count: usize,
}


#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateLobbyInfo {
    #[serde(default)]
    pub unlisted: bool,
}


/// Struct used as a temporary fix to not being able
/// to query elements in vectors with polodb.
/// This is just used for indexing.
#[derive(Debug, Serialize, Deserialize)]
pub struct RUsersLobby {
    pub user_id: i32,
    pub lobby_id: LobbyId
}


#[derive(Clone)]
pub struct BackendDb(polodb_core::Database);

impl BackendDb {
    pub fn cards_collection(&self) -> polodb_core::Collection<CardInfo> {
        self.0.collection("cards")
    }
    
    pub fn lobbies_collection(&self) -> polodb_core::Collection<Lobby> {
        self.0.collection("lobbies")
    }

    /// This is just used for indexing.
    pub fn lobby_users_collection(&self) -> polodb_core::Collection<RUsersLobby> {
        self.0.collection("lobby_users")
    }

    /// List lobbies, skipping unlisted ones
    pub fn paginate_lobby_list(&self, page: usize) -> Result<LobbyPageList, polodb_core::Error> {
        let lobbies = self.lobbies_collection();

        // count lobbies only if they aren't unlisted
        // output (print): [Document({"count": Int64(...)})]
        let count = lobbies.aggregate(vec![
            doc! {
                "$match": { "unlisted": false }
            },
            doc! {
                "$count": "count"
            }
        ]).run()?
            .collect::<polodb_core::Result<Vec<Document>>>()?
            [0].get_i64("count").unwrap();

        println!("Count: {}", count);

        // filter out unlisted lobbies
        let entries = lobbies.find(doc! { "unlisted": false })
            .skip((page * LOBBY_PAGE_SIZE) as u64)
            .limit(LOBBY_PAGE_SIZE as u64)
            .run()?
            .collect::<polodb_core::Result<Vec<Lobby>>>()?;

        for entry in entries.iter() {
            println!("Entry: {:?}", entry);
        }

        let entries = entries.iter()
            .map(Lobby::info)
            .collect();

        let page_count = (count as f64 / LOBBY_PAGE_SIZE as f64).ceil() as usize;

        Ok(LobbyPageList { entries, page, page_count })
    }

    pub fn collect_cards(&self) -> Result<Vec<Box<dyn Card>>, polodb_core::Error> {
        let cards_info = self.cards_collection()
            .find(doc! {})
            .run()?
            .collect::<polodb_core::Result<Vec<CardInfo>>>()?;

        Ok(cards_info.iter()
            .map(|info| info.make_card())
            .collect())
    }

    pub fn get_lobby_for_user(&self, account_id: i32) -> Option<Lobby> {
        // neither of these filters seem to work with arrays...
        // let filter = doc! { "users": account_id };
        // let filter = doc! { "users": { "$in": account_id } };

        // code to use if the filter works
        // match self.lobbies_collection().find_one(filter) {
        //     Ok(lobby) => lobby,
        //     Err(_) => None
        // }

        // use collection "lobby_users" for indexing
        if let Some(result) = self.lobby_users_collection()
                .find_one(doc! { "user_id": &account_id })
                .unwrap_or(None) {
            let lobby_id = result.lobby_id;
            let result = self.lobbies_collection()
                .find_one(doc! { "id": &lobby_id });

            result.unwrap_or(None)
        } else { None }
    }

    pub fn create_lobby(&self, creator_id: i32, info: &CreateLobbyInfo) -> Result<Lobby, actix_web::Error> {
        // check if user is already in a lobby
        if self.get_lobby_for_user(creator_id).is_some() {
            return Err(ErrorConflict("User is already in a lobby !"));
        }

        let txn = self.0.start_transaction().map_err(ErrorInternalServerError)?;

        let lobbies = self.lobbies_collection();

        let lobby_id = self.generate_lobby_id().map_err(ErrorInternalServerError)?;

        let mut lobby = Lobby::new(lobby_id.to_owned(), info.unlisted);
        lobby.users.insert(creator_id);

        // insert in collection
        lobbies.insert_one(&lobby).map_err(ErrorInternalServerError)?;

        // update index
        self.set_user_lobby_index(&lobby.id, creator_id).map_err(ErrorInternalServerError)?;

        txn.commit().map_err(ErrorInternalServerError)?;

        Ok(lobby)
    }

    pub fn join_lobby(&self, lobby_id: &LobbyId, account_id: i32) -> Result<Lobby, actix_web::Error> {
        let txn = self.0.start_transaction().map_err(ErrorInternalServerError)?;

        let lobbies = self.lobbies_collection();

        if self.get_lobby_for_user(account_id).is_some() {
            return Err(ErrorConflict("User is already in a lobby !"));
        }

        if let Some(mut lobby) = lobbies.find_one(doc! { "id": lobby_id })
                .map_err(ErrorInternalServerError)? {
            if lobby.users.len() + 1 > MAX_PLAYERS {
                return Err(ErrorBadRequest("Lobby is full !"));
            }

            // join lobby

            lobby.users.insert(account_id);

            // update in collection
            lobbies.update_one(doc! {
                "id": &lobby.id
            }, doc! {
                "$set": doc! {
                    "users": lobby.users.iter().cloned().collect::<Vec<i32>>(),
                }
            }).map_err(ErrorInternalServerError)?;

            // update index
            self.set_user_lobby_index(&lobby.id, account_id).map_err(ErrorInternalServerError)?;

            txn.commit().map_err(ErrorInternalServerError)?;

            Ok(lobby)
        } else {
            Err(ErrorNotFound("Lobby doesn't exist !"))
        }
    }

    pub fn leave_lobby(&self, account_id: i32) -> Result<Lobby, actix_web::Error> {
        let txn = self.0.start_transaction().map_err(ErrorInternalServerError)?;

        let lobbies = self.lobbies_collection();

        if let Some(mut lobby) = self.get_lobby_for_user(account_id) {
            if lobby.all_users_ready() && lobby.users.len() > 1 {
                return Err(ErrorConflict("Can't leave because all users are ready !"));
            }

            lobby.users.remove(&account_id);
            lobby.users_ready.remove(&account_id);

            if lobby.users.is_empty() {
                // remove lobby
                lobbies.delete_one(doc! {
                    "id": &lobby.id
                })
                .map_err(ErrorInternalServerError)?;

            } else {
                // update
                lobbies.update_one(doc! {
                    "id": &lobby.id
                }, doc! {
                    "$set": {
                        "users": lobby.users.iter().cloned().collect::<Vec<i32>>(),
                        "users_ready": lobby.users_ready.iter().cloned().collect::<Vec<i32>>()
                    }
                })
                .map_err(ErrorInternalServerError)?;
            }

            // update index
            self.unset_user_lobby_index(account_id).map_err(ErrorInternalServerError)?;

            txn.commit().map_err(ErrorInternalServerError)?;

            Ok(lobby)
        } else {
            Err(ErrorNotFound("User is not in a lobby !"))
        }
    }

    pub fn update_user_ready_state(&self, account_id: i32, ready: bool) -> Result<Lobby, actix_web::Error> {
        let lobbies = self.lobbies_collection();

        if let Some(mut lobby) = self.get_lobby_for_user(account_id) {

            if lobby.game_id.is_some() {
                return Err(ErrorConflict("Can't update because a game has already started !"))
            }

            if lobby.all_users_ready() && lobby.users.len() > 1 {
                return Err(ErrorConflict("Can't update because all users are ready !"));
            }

            if ready { lobby.users_ready.insert(account_id); }
            else { lobby.users_ready.remove(&account_id); }

            // update in collection
            lobbies.update_one(doc! {
                "id": &lobby.id
            }, doc! {
                "$set": doc! {
                    "users_ready": lobby.users_ready.iter().cloned().collect::<Vec<i32>>(),
                }
            }).map_err(ErrorInternalServerError)?;

            Ok(lobby)
        } else {
            Err(ErrorNotFound("User is not in a lobby !"))
        }
    }

    pub fn reset_users_ready_on_game_end(&self, game_id: &GameId) -> Result<(), actix_web::Error> {
        let lobbies = self.lobbies_collection();

        lobbies.update_one(doc! {
            "game_id": game_id.to_string()
        }, doc! {
            "$set": doc! {
                "users_ready": []
            }
        }).map_err(ErrorInternalServerError)?;

        Ok(())
    }

    fn set_user_lobby_index(&self, lobby_id: &LobbyId, account_id: i32) -> Result<(), polodb_core::Error> {
        let user_indexes = self.lobby_users_collection();

        if let Some(_) = user_indexes
                .find_one(doc! { "user_id": &account_id })? {
            user_indexes.update_one(doc! {
                "user_id": &account_id
            }, doc! {
                "lobby_id": &lobby_id
            })?;
        } else {
            user_indexes.insert_one(RUsersLobby { user_id: account_id, lobby_id: lobby_id.to_owned() })?;
        }

        Ok(())
    }

    fn unset_user_lobby_index(&self, account_id: i32) -> Result<(), polodb_core::Error> {
        let user_indexes = self.lobby_users_collection();

        user_indexes.delete_one(doc! { "user_id": &account_id })?;

        Ok(())
    }

    fn generate_lobby_id(&self) -> Result<LobbyId, String> {
        let lobbies = self.lobbies_collection();

        let mut i: usize = 0;
        // attempt to generate new random id
        // in case it was already generated, try again until limit is reached
        // this is very unlikely but possible
        while i < 10 {
            let id = nanoid!(LOBBY_ID_LEN, &LOBBY_ID_CHARS);

            let existing = lobbies.find_one(doc! {
                "id": &id
            }).map_err(|_| "Could not check existing lobbies")?;

            if existing.is_none() {
                return Ok(id);
            }

            i += 1;
        }

        Err("Unable to generate new lobby id !".to_string())
    }
}

impl Debug for BackendDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BackendDb(Database)")
    }
}


pub fn create_backend_db() -> Result<BackendDb, String> {
    let backend_db_path = std::env::var("BACKEND_DB_PATH").map_err(|_| "BACKEND_DB_PATH env var not set !")?;
    let db = Database::open_path(backend_db_path).map_err(|_| "Could not open db !")?;

    setup_backend_db(&db)?;

    Ok(BackendDb(db))
}


fn setup_backend_db(db: &Database) -> Result<(), String> {
    insert_cards_info_from_json(db)?;

    Ok(())
}


fn insert_cards_info_from_json(db: &Database) -> Result<(), String> {
    let collection: Collection<CardInfo> = db.collection("cards");   // creates collection if it doesn't exist

    let path = std::env::var("CARDS_FILE_PATH").map_err(|_| "CARDS_FILE_PATH not set !")?;

    if !Path::new(&path).exists() {
        return Err(format!("JSON file for cards not found ({})", path));
    }

    let file = File::open(&path).map_err(|_| "Could not open JSON file")?;
    let reader = BufReader::new(file);

    let cards_info: CardInfoList = serde_json::from_reader(reader).map_err(|_| "Error reading JSON file")?;

    collection.insert_many(cards_info.0).map_err(|e| e.to_string())?;

    Ok(())
}
