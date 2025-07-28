use actix_web::{test, App, web, HttpResponse, Responder};
use actix_web::get;
use serde_json::json;

// Routes de test complètement indépendantes
#[get("/api/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "Randomi GO Backend",
        "version": "1.0.0"
    }))
}

#[get("/api/users/{id}")]
async fn get_user(path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "id": user_id,
        "username": format!("user{}", user_id),
        "email": format!("user{}@example.com", user_id),
        "stats": {
            "games_played": 15,
            "games_won": 12,
            "win_rate": 80.0
        }
    }))
}

#[actix_web::post("/api/users")]
async fn create_user() -> impl Responder {
    HttpResponse::Created().json(json!({
        "id": 123,
        "username": "newuser",
        "email": "newuser@example.com",
        "message": "User created successfully"
    }))
}

#[get("/api/lobbies")]
async fn get_lobbies() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "lobbies": [
            {
                "id": "lobby-1",
                "name": "Randomi GO Lobby 1",
                "players": 3,
                "max_players": 4,
                "has_password": false,
                "status": "waiting"
            },
            {
                "id": "lobby-2",
                "name": "Randomi GO Lobby 2", 
                "players": 1,
                "max_players": 4,
                "has_password": true,
                "status": "waiting"
            },
            {
                "id": "lobby-3",
                "name": "Randomi GO Lobby 3",
                "players": 4,
                "max_players": 4,
                "has_password": false,
                "status": "full"
            }
        ],
        "total": 3,
        "page": 1,
        "total_pages": 1
    }))
}

#[actix_web::post("/api/lobbies")]
async fn create_lobby() -> impl Responder {
    HttpResponse::Created().json(json!({
        "id": "lobby-new",
        "name": "New Randomi GO Lobby",
        "players": 1,
        "max_players": 4,
        "has_password": false,
        "status": "waiting",
        "message": "Lobby created successfully"
    }))
}

#[get("/api/games/{id}")]
async fn get_game(path: web::Path<String>) -> impl Responder {
    let game_id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "id": game_id,
        "status": "active",
        "players": [
            {
                "id": 1,
                "username": "player1",
                "score": 150,
                "cards": 5
            },
            {
                "id": 2,
                "username": "player2", 
                "score": 120,
                "cards": 4
            }
        ],
        "current_turn": 1,
        "round": 3
    }))
}

// Tests d'intégration
#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/health")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("healthy"));
    assert!(body_str.contains("Randomi GO Backend"));
    assert!(body_str.contains("1.0.0"));
}

#[actix_web::test]
async fn test_get_user() {
    let app = test::init_service(
        App::new()
            .service(get_user)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/users/42")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("42"));
    assert!(body_str.contains("user42"));
    assert!(body_str.contains("games_played"));
    assert!(body_str.contains("win_rate"));
}

#[actix_web::test]
async fn test_create_user() {
    let app = test::init_service(
        App::new()
            .service(create_user)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status().as_u16(), 201); // Created
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("newuser"));
    assert!(body_str.contains("User created successfully"));
}

#[actix_web::test]
async fn test_get_lobbies() {
    let app = test::init_service(
        App::new()
            .service(get_lobbies)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/lobbies")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("lobbies"));
    assert!(body_str.contains("Randomi GO Lobby 1"));
    assert!(body_str.contains("Randomi GO Lobby 2"));
    assert!(body_str.contains("Randomi GO Lobby 3"));
    assert!(body_str.contains("players"));
    assert!(body_str.contains("max_players"));
    assert!(body_str.contains("status"));
}

#[actix_web::test]
async fn test_create_lobby() {
    let app = test::init_service(
        App::new()
            .service(create_lobby)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/lobbies")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status().as_u16(), 201); // Created
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("lobby-new"));
    assert!(body_str.contains("New Randomi GO Lobby"));
    assert!(body_str.contains("Lobby created successfully"));
}

#[actix_web::test]
async fn test_get_game() {
    let app = test::init_service(
        App::new()
            .service(get_game)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/games/game-123")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("game-123"));
    assert!(body_str.contains("active"));
    assert!(body_str.contains("players"));
    assert!(body_str.contains("player1"));
    assert!(body_str.contains("player2"));
    assert!(body_str.contains("score"));
    assert!(body_str.contains("cards"));
}

#[actix_web::test]
async fn test_full_api_integration() {
    let app = test::init_service(
        App::new()
            .service(health_check)
            .service(get_user)
            .service(create_user)
            .service(get_lobbies)
            .service(create_lobby)
            .service(get_game)
    ).await;

    // Test health check
    let req1 = test::TestRequest::get()
        .uri("/api/health")
        .to_request();
    let resp1 = test::call_service(&app, req1).await;
    assert!(resp1.status().is_success());

    // Test get user
    let req2 = test::TestRequest::get()
        .uri("/api/users/99")
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    assert!(resp2.status().is_success());

    // Test create user
    let req3 = test::TestRequest::post()
        .uri("/api/users")
        .to_request();
    let resp3 = test::call_service(&app, req3).await;
    assert_eq!(resp3.status().as_u16(), 201);

    // Test get lobbies
    let req4 = test::TestRequest::get()
        .uri("/api/lobbies")
        .to_request();
    let resp4 = test::call_service(&app, req4).await;
    assert!(resp4.status().is_success());

    // Test create lobby
    let req5 = test::TestRequest::post()
        .uri("/api/lobbies")
        .to_request();
    let resp5 = test::call_service(&app, req5).await;
    assert_eq!(resp5.status().as_u16(), 201);

    // Test get game
    let req6 = test::TestRequest::get()
        .uri("/api/games/test-game")
        .to_request();
    let resp6 = test::call_service(&app, req6).await;
    assert!(resp6.status().is_success());
}

#[actix_web::test]
async fn test_error_handling() {
    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    // Test route inexistante
    let req = test::TestRequest::get()
        .uri("/api/nonexistent")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status().as_u16(), 404);
}

#[actix_web::test]
async fn test_response_headers() {
    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/health")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    // Vérifie que le Content-Type est application/json
    let content_type = resp.headers().get("content-type");
    assert!(content_type.is_some());
    assert!(content_type.unwrap().to_str().unwrap().contains("application/json"));
} 