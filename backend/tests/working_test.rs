use actix_web::{test, App, HttpResponse, Responder};
use actix_web::get;

// Route de test simple
#[get("/test")]
async fn test_route() -> impl Responder {
    HttpResponse::Ok().json("Hello from test route!")
}

// Route de test avec paramètre
#[get("/test/{name}")]
async fn test_route_with_param(path: actix_web::web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().json(format!("Hello {} from test route!", name))
}

// Route de test POST
#[actix_web::post("/test")]
async fn test_post_route() -> impl Responder {
    HttpResponse::Created().json("POST request successful!")
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

#[actix_web::test]
async fn test_route_with_parameters() {
    let app = test::init_service(
        App::new()
            .service(test_route_with_param)
    ).await;

    let req = test::TestRequest::get()
        .uri("/test/World")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Hello World from test route!"));
}

#[actix_web::test]
async fn test_post_route() {
    let app = test::init_service(
        App::new()
            .service(test_post_route)
    ).await;

    let req = test::TestRequest::post()
        .uri("/test")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status().as_u16(), 201); // Created
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("POST request successful!"));
}

#[actix_web::test]
async fn test_multiple_routes() {
    let app = test::init_service(
        App::new()
            .service(test_route)
            .service(test_route_with_param)
            .service(test_post_route)
    ).await;

    // Test GET route
    let req1 = test::TestRequest::get()
        .uri("/test")
        .to_request();
    let resp1 = test::call_service(&app, req1).await;
    assert!(resp1.status().is_success());

    // Test POST route
    let req2 = test::TestRequest::post()
        .uri("/test")
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    assert_eq!(resp2.status().as_u16(), 201);

    // Test route with parameter
    let req3 = test::TestRequest::get()
        .uri("/test/Rust")
        .to_request();
    let resp3 = test::call_service(&app, req3).await;
    assert!(resp3.status().is_success());
}

#[actix_web::test]
async fn test_404_for_unknown_route() {
    let app = test::init_service(
        App::new()
            .service(test_route)
    ).await;

    let req = test::TestRequest::get()
        .uri("/unknown-route")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status().as_u16(), 404);
} 