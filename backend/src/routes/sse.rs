use std::collections::HashSet;
use std::{sync::Arc, time::Duration};

use actix_web::{get, web, HttpMessage, HttpRequest, Responder};
use actix_web::rt::time::interval;
use actix_web_lab::{
    sse::{self, Sse},
    util::InfallibleStream,
};
use futures_util::future;
use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use serde_derive::Serialize;

use crate::database::models::Friend;
use crate::routes::game::Lobby;
use crate::GameId;


#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum SseMessage {
    FriendRequest { request_id: i32, user: i32, status: i32 },
    LobbyUserListChange { users: HashSet<i32> },
    LobbyUserReadyChange { user: i32, ready: bool },
    GameStarted { game_id: GameId }
}

impl SseMessage {
    pub async fn send(&self, tx: &mpsc::Sender<sse::Event>) -> Result<(), mpsc::error::SendError<sse::Event>> {
        tx.send(sse::Data::new_json(&self).unwrap().into()).await
    }
}


#[derive(Debug, Clone)]
pub struct SseClient {
    pub account_id: i32,
    pub tx: mpsc::Sender<sse::Event>
}

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<SseClient>,
}

impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Broadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.
    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();

        let mut ok_clients = Vec::new();

        for client in clients {
            if client.tx
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self, account_id: i32) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(10);

        tx.send(sse::Data::new("connected").into()).await.unwrap();

        self.inner.lock().clients.push(SseClient { account_id, tx });

        Sse::from_infallible_receiver(rx)
    }

    /// Send message to client that is the subject of the friend request
    pub async fn notify_friend_request_update(&self, friend_request: &Friend) {
        let clients = self.inner.lock().clients.clone();

        for client in clients.iter() {
            if client.account_id == friend_request.account2 {
                let msg = SseMessage::FriendRequest {
                    request_id: friend_request.id,
                    user: friend_request.account1,
                    status: friend_request.status
                };
                let _ = msg.send(&client.tx).await;
                return;
            }
        }
    }

    pub async fn notify_lobby_user_list_update(&self, lobby: &Lobby, skip_id: i32) {
        let clients = self.inner.lock().clients.clone();
        
        let msg = SseMessage::LobbyUserListChange { users: lobby.users.clone() };

        let send_futures = clients
            .iter()
            .filter(|client|
                client.account_id != skip_id
                && lobby.users.contains(&client.account_id))
            .map(|client| msg.send(&client.tx));

        // try to send to all clients
        let _ = future::join_all(send_futures).await;

    }

    pub async fn notify_lobby_user_ready(&self, lobby: &Lobby, account_id: i32, ready: bool) {
        let clients = self.inner.lock().clients.clone();
        
        let msg = SseMessage::LobbyUserReadyChange { user: account_id, ready };

        let send_futures = clients
            .iter()
            .filter(|client|
                client.account_id != account_id
                && lobby.users.contains(&client.account_id))
            .map(|client| msg.send(&client.tx));

        // try to send to all clients
        let _ = future::join_all(send_futures).await;

    }

    pub async fn notify_lobby_game_started(&self, lobby: &Lobby) {
        let clients = self.inner.lock().clients.clone();
        
        let msg = SseMessage::GameStarted { game_id: lobby.game_id.unwrap() };

        let send_futures = clients
            .iter()
            .filter(|client|
                lobby.users.contains(&client.account_id))
            .map(|client| msg.send(&client.tx));

        // try to send to all clients
        let _ = future::join_all(send_futures).await;

    }

    // /// Broadcasts `msg` to all clients.
    // pub async fn broadcast(&self, msg: &str) {
    //     let clients = self.inner.lock().clients.clone();

    //     let send_futures = clients
    //         .iter()
    //         .map(|client| client.tx.send(sse::Data::new(msg).into()));

    //     // try to send to all clients, ignoring failures
    //     // disconnected clients will get swept up by `remove_stale_clients`
    //     let _ = future::join_all(send_futures).await;
    // }
}


#[get("/events")]
async fn event_stream(
    req: HttpRequest,
    broadcaster: web::Data<Broadcaster>
) -> impl Responder {
    let account_id: i32 = req.extensions().get::<i32>()
                             .unwrap()
                             .clone();
    
    broadcaster.new_client(account_id).await
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(event_stream);
}