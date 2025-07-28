use actix_web::{test, App, HttpResponse, Responder};
use actix_web::get;

// Route de test simple
#[get("/test")]
async fn test_route() -> impl Responder {
    HttpResponse::Ok().json("Hello from test route!")
}

#[actix_web::test]
async fn test_basic_math() {
    assert_eq!(2 + 2, 4);
}

#[actix_web::test]
async fn test_actix_web_integration() {
    // Test simple pour vérifier que l'app démarre
    let app = test::init_service(
        App::new()
            .service(test_route)
    ).await;

    // Test de la route simple
    let req = test::TestRequest::get()
        .uri("/test")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Hello from test route!"));
}