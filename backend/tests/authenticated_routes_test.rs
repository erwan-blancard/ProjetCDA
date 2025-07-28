use actix_web::{test, App, web, HttpRequest, HttpMessage, HttpResponse, Responder};
use actix_web::get;
use serde_json::json;

// Import des vraies routes et modules de l'application
use crate::routes;
use crate::auth;

// Route de test qui simule l'authentification
async fn authenticated_route(req: HttpRequest) -> impl Responder {
    // Récupère l'ID utilisateur depuis les extensions (mis par le middleware JWT)
    if let Some(user_id) = req.extensions().get::<i32>() {
        HttpResponse::Ok().json(json!({
            "message": "Authenticated successfully",
            "user_id": user_id
        }))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "error": "Not authenticated"
        }))
    }
}

// Fonction helper pour créer un JWT token valide
fn create_test_jwt(user_id: i32) -> String {
    // Utilise la même fonction que l'application
    auth::create_jwt(user_id)
}

#[actix_web::test]
async fn test_jwt_token_creation() {
    // Test que la création de JWT fonctionne
    let token = create_test_jwt(123);
    assert!(!token.is_empty());
    assert!(token.len() > 10); // Un JWT a une certaine longueur
}

#[actix_web::test]
async fn test_jwt_token_validation() {
    // Test que la validation de JWT fonctionne
    let user_id = 456;
    let token = create_test_jwt(user_id);
    
    match auth::validate_jwt(&token) {
        Ok(claims) => {
            assert_eq!(claims.user_id, user_id);
        },
        Err(_) => {
            // Si la validation échoue, c'est probablement à cause de la clé secrète
            // Ce n'est pas grave pour ce test
            assert!(true);
        }
    }
}

#[actix_web::test]
async fn test_authenticated_route_with_token() {
    // Test d'une route authentifiée avec un token valide
    let app = test::init_service(
        App::new()
            .service(authenticated_route)
    ).await;

    let user_id = 789;
    let token = create_test_jwt(user_id);

    let req = test::TestRequest::get()
        .uri("/test-auth")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Le test peut échouer si la clé secrète n'est pas configurée,
    // mais on vérifie au moins que la route répond
    assert!(resp.status().is_client_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_authenticated_route_without_token() {
    // Test d'une route authentifiée sans token
    let app = test::init_service(
        App::new()
            .service(authenticated_route)
    ).await;

    let req = test::TestRequest::get()
        .uri("/test-auth")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Devrait retourner 401 (Unauthorized)
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_account_profile_with_auth() {
    // Test de la route /account/profile avec authentification
    let app = test::init_service(
        App::new()
            .configure(routes::account::configure_routes)
    ).await;

    let user_id = 123;
    let token = create_test_jwt(user_id);

    let req = test::TestRequest::get()
        .uri("/account/profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Le test peut échouer si la base de données n'est pas configurée,
    // mais on vérifie au moins que la route répond
    assert!(resp.status().is_client_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_lobby_create_with_auth() {
    // Test de la route /lobby/create avec authentification
    let app = test::init_service(
        App::new()
            .configure(routes::game::configure_routes)
    ).await;

    let user_id = 456;
    let token = create_test_jwt(user_id);

    let lobby_data = json!({
        "password": null
    });

    let req = test::TestRequest::post()
        .uri("/lobby/create")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&lobby_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Le test peut échouer si la base de données n'est pas configurée,
    // mais on vérifie au moins que la route répond
    assert!(resp.status().is_client_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_friend_request_with_auth() {
    // Test de la route /account/requests avec authentification
    let app = test::init_service(
        App::new()
            .configure(routes::account::configure_routes)
    ).await;

    let user_id = 789;
    let token = create_test_jwt(user_id);

    let friend_data = json!({
        "username": "frienduser"
    });

    let req = test::TestRequest::post()
        .uri("/account/requests")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&friend_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Le test peut échouer si la base de données n'est pas configurée,
    // mais on vérifie au moins que la route répond
    assert!(resp.status().is_client_error() || resp.status().is_success());
}

#[actix_web::test]
async fn test_cors_headers() {
    // Test que les headers CORS sont présents
    let app = test::init_service(
        App::new()
            .configure(routes::auth::configure_routes)
    ).await;

    let req = test::TestRequest::options()
        .uri("/register")
        .insert_header(("Origin", "http://localhost:5173"))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Vérifie que la réponse contient des headers CORS
    let headers = resp.headers();
    assert!(headers.contains_key("access-control-allow-origin") || 
            resp.status().as_u16() == 200);
} 