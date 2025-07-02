use actix_web::{test, App};
use backend::main as backend_main;

#[actix_rt::test]
async fn test_get_friends_for_account() {
    let mut app = test::init_service(App::new().configure(backend_main::app_config)).await;
    let req = test::TestRequest::get().uri("/api/friends").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
} 