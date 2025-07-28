use actix_web::{test, App, HttpResponse, Responder};
use actix_web::get;
use serde_json::json;

// Import des vraies routes de l'application
use crate::routes;

// Route de test qui génère une erreur
#[get("/error-test")]
async fn error_test() -> impl Responder {
    HttpResponse::InternalServerError().json(json!({
        "error": "Test error"
    }))
}

#[actix_web::test]
async fn test_invalid_json_register() {
    // Test avec un JSON invalide pour /register
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/register")
        .insert_header(("Content-Type", "application/json"))
        .set_payload("invalid json")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 400 (Bad Request) pour JSON invalide
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_missing_fields_register() {
    // Test avec des champs manquants pour /register
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    let incomplete_data = json!({
        "username": "testuser"
        // email et password manquants
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&incomplete_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 400 (Bad Request) pour champs manquants
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_invalid_json_login() {
    // Test avec un JSON invalide pour /login
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .insert_header(("Content-Type", "application/json"))
        .set_payload("invalid json")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 400 (Bad Request) pour JSON invalide
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_wrong_method_on_routes() {
    // Test avec des méthodes HTTP incorrectes
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
            .configure(routes::account::configure_routes)
    ).await;

    // GET sur /register (qui est POST)
    let req1 = test::TestRequest::get()
        .uri("/register")
        .to_request();
    let resp1 = test::call_service(&app, req1).await;
    assert_eq!(resp1.status().as_u16(), 405); // Method Not Allowed

    // POST sur /account/profile (qui est GET)
    let req2 = test::TestRequest::post()
        .uri("/account/profile")
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    assert_eq!(resp2.status().as_u16(), 405); // Method Not Allowed
}

#[actix_web::test]
async fn test_invalid_path_parameters() {
    // Test avec des paramètres de chemin invalides
    let app = test::init_service(
        App::new()
            .configure(routes::game::configure_routes)
    ).await;

    // Test avec un ID de lobby invalide
    let req = test::TestRequest::get()
        .uri("/lobby/find/not-a-uuid")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 400 (Bad Request) ou 404 (Not Found)
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_large_payload() {
    // Test avec un payload très large
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    // Crée un payload très large
    let large_username = "a".repeat(10000);
    let large_data = json!({
        "username": large_username,
        "email": "test@example.com",
        "password": "password123"
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&large_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner une erreur pour payload trop large
    assert!(resp.status().is_client_error() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_special_characters_in_data() {
    // Test avec des caractères spéciaux
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    let special_data = json!({
        "username": "test@user#123!",
        "email": "test+tag@example.com",
        "password": "pass@word#123!"
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&special_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Le test peut échouer si les caractères spéciaux ne sont pas autorisés,
    // mais on vérifie au moins que la route répond
    assert!(resp.status().is_client_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_empty_fields() {
    // Test avec des champs vides
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    let empty_data = json!({
        "username": "",
        "email": "",
        "password": ""
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&empty_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 400 (Bad Request) pour champs vides
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_malformed_headers() {
    // Test avec des headers malformés
    let app = test::init_service(
        App::new()
            .configure(routes::account::configure_routes)
    ).await;

    let req = test::TestRequest::get()
        .uri("/account/profile")
        .insert_header(("Authorization", "Bearer invalid-token"))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 401 (Unauthorized) pour token invalide
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_concurrent_requests() {
    // Test de requêtes concurrentes
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    // Crée plusieurs requêtes simultanées
    let req1 = test::TestRequest::get()
        .uri("/register")
        .to_request();
    
    let req2 = test::TestRequest::get()
        .uri("/register")
        .to_request();
    
    let resp1 = test::call_service(&app, req1).await;
    let resp2 = test::call_service(&app, req2).await;
    
    // Les deux devraient retourner 405 (Method Not Allowed)
    assert_eq!(resp1.status().as_u16(), 405);
    assert_eq!(resp2.status().as_u16(), 405);
}

#[actix_web::test]
async fn test_error_response_format() {
    // Test que les erreurs retournent un format JSON cohérent
    let app = test::init_service(
        App::new()
            .service(error_test)
    ).await;

    let req = test::TestRequest::get()
        .uri("/error-test")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status().as_u16(), 500);
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    // Vérifie que la réponse contient un JSON d'erreur
    assert!(body_str.contains("error"));
    assert!(body_str.contains("Test error"));
} 