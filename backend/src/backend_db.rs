use std::fmt::Debug;
use std::{fs::File, io::BufReader, path::Path};

use actix_web::error::{ErrorBadRequest, ErrorConflict, ErrorInternalServerError, ErrorNotFound};
use polodb_core::bson::doc;
use polodb_core::{Collection, CollectionT, Database};

use crate::server::game::card_info::{CardInfo, CardInfoList};
use crate::routes::game::{Lobby, LobbyId};
use crate::server::game::cards::card::Card;
use crate::server::game::game::MAX_PLAYERS;
use crate::GameId;


#[derive(Clone)]
pub struct BackendDb(polodb_core::Database);

impl BackendDb {
    pub fn cards_collection(&self) -> polodb_core::Collection<CardInfo> {
        self.0.collection("cards")
    }
    
    pub fn lobbies_collection(&self) -> polodb_core::Collection<Lobby> {
        self.0.collection("lobbies")
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
        None
    }

    pub fn join_lobby(&self, lobby_id: &LobbyId, account_id: i32) -> Result<Lobby, actix_web::Error> {
        let lobbies = self.lobbies_collection();

        if self.get_lobby_for_user(account_id).is_some() {
            return Err(ErrorConflict("User is already in a lobby !"));
        }

        if let Some(mut lobby) = lobbies.find_one(doc! { "id": lobby_id })
                .map_err(|e| ErrorInternalServerError(e.to_string()))? {
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
            }).map_err(|e| ErrorInternalServerError(e.to_string()))?;

            Ok(lobby)
        } else {
            Err(ErrorNotFound("Lobby doesn't exist !"))
        }
    }

    pub fn leave_lobby(&self, account_id: i32) -> Result<Lobby, actix_web::Error> {
        let lobbies = self.lobbies_collection();

        if let Some(mut lobby) = self.get_lobby_for_user(account_id) {
            if lobby.all_users_ready() {
                return Err(ErrorConflict("Can't leave because all users are ready !"));
            }

            lobby.users.remove(&account_id);
            lobby.users_ready.remove(&account_id);

            if lobby.users.is_empty() {
                // remove lobby
                lobbies.delete_one(doc! {
                    "id": &lobby.id
                })
                .map_err(|e| ErrorInternalServerError(e.to_string()))?;

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
                .map_err(|e| ErrorInternalServerError(e.to_string()))?;
            }

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
            }).map_err(|e| ErrorInternalServerError(e.to_string()))?;

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
            "users_ready": []
        }).map_err(|e| ErrorInternalServerError(e.to_string()))?;

        Ok(())
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
