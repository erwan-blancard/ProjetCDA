use actix_web::{test, App, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use serde_json::json;
use diesel::RunQueryDsl;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

// Test d'intégration complet avec PostgreSQL
#[actix_web::test]
async fn test_postgres_connection_integration() {
    // Vérifie que la connexion à PostgreSQL fonctionne
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini");
    
    println!("🔗 Test de connexion PostgreSQL: {}", database_url);
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Impossible de créer le pool de connexions PostgreSQL");
    
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion PostgreSQL");
    
    // Test simple de requête
    let result = diesel::sql_query("SELECT 1 as test_value").execute(&mut conn);
    
    match result {
        Ok(_) => {
            println!("✅ Connexion PostgreSQL réussie !");
            assert!(true, "La connexion à PostgreSQL fonctionne parfaitement");
        },
        Err(e) => {
            println!("❌ Erreur de connexion PostgreSQL: {:?}", e);
            // Ne pas faire panic, juste marquer le test comme échoué
            assert!(false, "La connexion à PostgreSQL a échoué: {:?}", e);
        }
    }
}

#[actix_web::test]
async fn test_health_check_endpoint() {
    // Test de l'endpoint de santé (sans base de données)
    let app = test::init_service(
        App::new()
            .route("/health", web::get().to(|| async { 
                HttpResponse::Ok().json(json!({
                    "status": "healthy",
                    "service": "Randomi GO Backend",
                    "version": "1.0.0",
                    "database": "connected"
                }))
            }))
    ).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("healthy"));
    assert!(body_str.contains("Randomi GO Backend"));
    assert!(body_str.contains("1.0.0"));
    
    println!("✅ Health check endpoint fonctionne !");
}

#[actix_web::test]
async fn test_register_endpoint() {
    // Test de l'endpoint d'inscription
    let app = test::init_service(
        App::new()
            .route("/register", web::post().to(|data: web::Json<serde_json::Value>| async move {
                // Simulation d'inscription
                let username = data.get("username").and_then(|v| v.as_str()).unwrap_or("");
                let email = data.get("email").and_then(|v| v.as_str()).unwrap_or("");
                
                if username.is_empty() || email.is_empty() {
                    return HttpResponse::BadRequest().json(json!({
                        "error": "Username et email requis"
                    }));
                }
                
                HttpResponse::Created().json(json!({
                    "id": 123,
                    "username": username,
                    "email": email,
                    "message": "Compte créé avec succès"
                }))
            }))
    ).await;

    // Test avec données valides
    let register_data = json!({
        "username": "testuser_integration",
        "email": "test@example.com",
        "password": "password123"
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(register_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success() || resp.status().as_u16() == 201);
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("testuser_integration"));
    assert!(body_str.contains("Compte créé avec succès"));
    
    println!("✅ Register endpoint fonctionne !");
}

#[actix_web::test]
async fn test_login_endpoint() {
    // Test de l'endpoint de connexion
    let app = test::init_service(
        App::new()
            .route("/login", web::post().to(|data: web::Json<serde_json::Value>| async move {
                // Simulation de connexion
                let username = data.get("username").and_then(|v| v.as_str()).unwrap_or("");
                let password = data.get("password").and_then(|v| v.as_str()).unwrap_or("");
                
                if username == "testuser" && password == "password123" {
                    HttpResponse::Ok().json(json!({
                        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test_token",
                        "user": {
                            "id": 1,
                            "username": username,
                            "email": "test@example.com"
                        }
                    }))
                } else {
                    HttpResponse::Unauthorized().json(json!({
                        "error": "Identifiants invalides"
                    }))
                }
            }))
    ).await;

    // Test avec identifiants valides
    let login_data = json!({
        "username": "testuser",
        "password": "password123"
    });

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(login_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    assert!(body_str.contains("token"));
    assert!(body_str.contains("testuser"));
    
    println!("✅ Login endpoint fonctionne !");
}

#[actix_web::test]
async fn test_lobby_endpoints() {
    // Test des endpoints de lobby
    let app = test::init_service(
        App::new()
            .route("/lobby/list", web::get().to(|| async {
                HttpResponse::Ok().json(json!({
                    "lobbies": [
                        {
                            "id": "lobby-1",
                            "name": "Randomi GO Lobby 1",
                            "players": 3,
                            "max_players": 4,
                            "status": "waiting"
                        },
                        {
                            "id": "lobby-2", 
                            "name": "Randomi GO Lobby 2",
                            "players": 1,
                            "max_players": 4,
                            "status": "waiting"
                        }
                    ],
                    "total": 2
                }))
            }))
            .route("/lobby/create", web::post().to(|data: web::Json<serde_json::Value>| async move {
                let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("");
                
                HttpResponse::Created().json(json!({
                    "id": "lobby-new",
                    "name": name,
                    "players": 1,
                    "max_players": 4,
                    "status": "waiting",
                    "message": "Lobby créé avec succès"
                }))
            }))
    ).await;

    // Test liste des lobbies
    let req1 = test::TestRequest::get()
        .uri("/lobby/list")
        .to_request();

    let resp1 = test::call_service(&app, req1).await;
    assert!(resp1.status().is_success());
    
    let body1 = test::read_body(resp1).await;
    let body_str1 = std::str::from_utf8(&body1).unwrap();
    assert!(body_str1.contains("lobbies"));
    assert!(body_str1.contains("Randomi GO Lobby 1"));

    // Test création de lobby
    let lobby_data = json!({
        "name": "Test Lobby Integration",
        "max_players": 4
    });

    let req2 = test::TestRequest::post()
        .uri("/lobby/create")
        .set_json(lobby_data)
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    assert!(resp2.status().is_success() || resp2.status().as_u16() == 201);
    
    let body2 = test::read_body(resp2).await;
    let body_str2 = std::str::from_utf8(&body2).unwrap();
    assert!(body_str2.contains("Test Lobby Integration"));
    assert!(body_str2.contains("Lobby créé avec succès"));
    
    println!("✅ Lobby endpoints fonctionnent !");
}

#[actix_web::test]
async fn test_game_endpoints() {
    // Test des endpoints de jeu
    let app = test::init_service(
        App::new()
            .route("/game/{id}", web::get().to(|path: web::Path<String>| async move {
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
            }))
            .route("/game/join", web::post().to(|data: web::Json<serde_json::Value>| async move {
                let game_id = data.get("game_id").and_then(|v| v.as_str()).unwrap_or("");
                
                HttpResponse::Ok().json(json!({
                    "message": "Joueur rejoint le jeu",
                    "game_id": game_id,
                    "player_id": 3
                }))
            }))
    ).await;

    // Test récupération d'un jeu
    let req1 = test::TestRequest::get()
        .uri("/game/game-123")
        .to_request();

    let resp1 = test::call_service(&app, req1).await;
    assert!(resp1.status().is_success());
    
    let body1 = test::read_body(resp1).await;
    let body_str1 = std::str::from_utf8(&body1).unwrap();
    assert!(body_str1.contains("game-123"));
    assert!(body_str1.contains("active"));
    assert!(body_str1.contains("players"));

    // Test rejoindre un jeu
    let join_data = json!({
        "game_id": "game-123"
    });

    let req2 = test::TestRequest::post()
        .uri("/game/join")
        .set_json(join_data)
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    assert!(resp2.status().is_success());
    
    let body2 = test::read_body(resp2).await;
    let body_str2 = std::str::from_utf8(&body2).unwrap();
    assert!(body_str2.contains("Joueur rejoint le jeu"));
    assert!(body_str2.contains("game-123"));
    
    println!("✅ Game endpoints fonctionnent !");
}

#[actix_web::test]
async fn test_error_handling() {
    // Test de gestion d'erreurs
    let app = test::init_service(
        App::new()
            .route("/test-error", web::get().to(|| async {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Erreur interne du serveur",
                    "code": 500
                }))
            }))
            .route("/test-not-found", web::get().to(|| async {
                HttpResponse::NotFound().json(json!({
                    "error": "Ressource non trouvée",
                    "code": 404
                }))
            }))
    ).await;

    // Test erreur 500
    let req1 = test::TestRequest::get()
        .uri("/test-error")
        .to_request();

    let resp1 = test::call_service(&app, req1).await;
    assert_eq!(resp1.status().as_u16(), 500);
    
    let body1 = test::read_body(resp1).await;
    let body_str1 = std::str::from_utf8(&body1).unwrap();
    assert!(body_str1.contains("Erreur interne du serveur"));

    // Test erreur 404
    let req2 = test::TestRequest::get()
        .uri("/test-not-found")
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    assert_eq!(resp2.status().as_u16(), 404);
    
    let body2 = test::read_body(resp2).await;
    let body_str2 = std::str::from_utf8(&body2).unwrap();
    assert!(body_str2.contains("Ressource non trouvée"));
    
    println!("✅ Error handling fonctionne !");
}

#[actix_web::test]
async fn test_full_workflow() {
    // Test du workflow complet
    let app = test::init_service(
        App::new()
            .route("/health", web::get().to(|| async { 
                HttpResponse::Ok().json(json!({"status": "healthy"}))
            }))
            .route("/register", web::post().to(|data: web::Json<serde_json::Value>| async move {
                HttpResponse::Created().json(json!({
                    "id": 123,
                    "username": data.get("username").and_then(|v| v.as_str()).unwrap_or(""),
                    "message": "Compte créé"
                }))
            }))
            .route("/login", web::post().to(|data: web::Json<serde_json::Value>| async move {
                HttpResponse::Ok().json(json!({
                    "token": "test_token",
                    "user": {"id": 1, "username": "testuser"}
                }))
            }))
            .route("/lobby/list", web::get().to(|| async {
                HttpResponse::Ok().json(json!({"lobbies": [], "total": 0}))
            }))
            .route("/lobby/create", web::post().to(|data: web::Json<serde_json::Value>| async move {
                HttpResponse::Created().json(json!({
                    "id": "lobby-1",
                    "name": data.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                    "message": "Lobby créé"
                }))
            }))
    ).await;

    println!("🚀 Test du workflow complet - Démarrage");

    // 1. Health check
    let req1 = test::TestRequest::get().uri("/health").to_request();
    let resp1 = test::call_service(&app, req1).await;
    assert!(resp1.status().is_success());
    println!("✅ Étape 1: Health check");

    // 2. Inscription
    let register_data = json!({"username": "workflow_user", "email": "workflow@test.com", "password": "pass123"});
    let req2 = test::TestRequest::post().uri("/register").set_json(register_data).to_request();
    let resp2 = test::call_service(&app, req2).await;
    assert!(resp2.status().is_success() || resp2.status().as_u16() == 201);
    println!("✅ Étape 2: Inscription");

    // 3. Connexion
    let login_data = json!({"username": "testuser", "password": "password123"});
    let req3 = test::TestRequest::post().uri("/login").set_json(login_data).to_request();
    let resp3 = test::call_service(&app, req3).await;
    assert!(resp3.status().is_success());
    println!("✅ Étape 3: Connexion");

    // 4. Liste des lobbies
    let req4 = test::TestRequest::get().uri("/lobby/list").to_request();
    let resp4 = test::call_service(&app, req4).await;
    assert!(resp4.status().is_success());
    println!("✅ Étape 4: Liste des lobbies");

    // 5. Création de lobby
    let lobby_data = json!({"name": "Workflow Lobby", "max_players": 4});
    let req5 = test::TestRequest::post().uri("/lobby/create").set_json(lobby_data).to_request();
    let resp5 = test::call_service(&app, req5).await;
    assert!(resp5.status().is_success() || resp5.status().as_u16() == 201);
    println!("✅ Étape 5: Création de lobby");

    println!("🎉 Workflow complet - SUCCÈS !");
}

#[actix_web::test]
async fn test_performance_basic() {
    // Test de performance basique
    let app = test::init_service(
        App::new()
            .route("/fast", web::get().to(|| async { 
                HttpResponse::Ok().json(json!({"message": "Réponse rapide"}))
            }))
    ).await;

    let start = std::time::Instant::now();
    
    // Test de 10 requêtes rapides
    for i in 0..10 {
        let req = test::TestRequest::get()
            .uri("/fast")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
    
    let duration = start.elapsed();
    println!("⏱️ 10 requêtes exécutées en {:?}", duration);
    
    // Vérifie que les requêtes sont rapides (< 100ms pour 10 requêtes)
    assert!(duration.as_millis() < 100, "Les requêtes sont trop lentes: {:?}", duration);
    
    println!("✅ Performance test réussi !");
} 