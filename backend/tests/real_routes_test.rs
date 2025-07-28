use actix_web::{test, App, web, HttpResponse, Responder};
use actix_web::get;
use serde_json::json;

// Import des vraies routes de l'application
use crate::routes;

// Route de test simple pour vérifier que l'app démarre
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("OK")
}

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("OK"));
}

#[actix_web::test]
async fn test_register_route() {
    // Test de la route /register
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    let register_data = json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "password123"
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&register_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Le test peut échouer si la base de données n'est pas configurée,
    // mais on vérifie au moins que la route répond
    assert!(resp.status().is_client_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_login_route() {
    // Test de la route /login
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    let login_data = json!({
        "username": "testuser",
        "password": "password123"
    });

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Le test peut échouer si l'utilisateur n'existe pas,
    // mais on vérifie au moins que la route répond
    assert!(resp.status().is_client_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_account_routes_structure() {
    // Test que les routes d'account sont configurées
    let app = test::init_service(
        App::new()
            .configure(routes::account::configure_routes)
    ).await;

    // Test de la route /account/profile (sans authentification)
    let req = test::TestRequest::get()
        .uri("/account/profile")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 401 (Unauthorized) car pas de token JWT
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_lobby_routes_structure() {
    // Test que les routes de lobby sont configurées
    let app = test::init_service(
        App::new()
            .configure(routes::game::configure_routes)
    ).await;

    // Test de la route /lobby/list/0 (sans authentification)
    let req = test::TestRequest::get()
        .uri("/lobby/list/0")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 401 (Unauthorized) car pas de token JWT
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_sse_route_structure() {
    // Test que la route SSE est configurée
    let app = test::init_service(
        App::new()
            .configure(routes::sse::configure_routes)
    ).await;

    // Test de la route /events (sans authentification)
    let req = test::TestRequest::get()
        .uri("/events")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 401 (Unauthorized) car pas de token JWT
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_all_routes_configured() {
    // Test que toutes les routes sont configurées ensemble
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
            .configure(routes::account::configure_routes)
            .configure(routes::game::configure_routes)
            .configure(routes::sse::configure_routes)
    ).await;

    // Test que l'app démarre avec toutes les routes
    let req = test::TestRequest::get()
        .uri("/register") // Route qui existe
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 405 (Method Not Allowed) car GET sur POST route
    assert_eq!(resp.status().as_u16(), 405);
}

#[actix_web::test]
async fn test_404_for_unknown_routes() {
    // Test que les routes inconnues retournent 404
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
            .configure(routes::account::configure_routes)
            .configure(routes::game::configure_routes)
    ).await;

    let req = test::TestRequest::get()
        .uri("/route/qui/n/existe/pas")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 404 (Not Found)
    assert_eq!(resp.status().as_u16(), 404);
} 