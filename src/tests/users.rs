#[allow(unused_imports)]
use crate::db::connection::DbConnection;
#[allow(unused_imports)]
use crate::models::user::{CreateUserRequest, User};
#[allow(unused_imports)]
use crate::routes::users::users_routes;
#[allow(unused_imports)]
use actix_web::{test, web::Data, App};
#[allow(unused_imports)]
use reqwest::Response;
#[allow(unused_imports)]
use serde_json::json;
#[allow(unused_imports)]
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
#[actix_web::test]
pub async fn test_create_user_unit() {
    let db = Arc::new(
        DbConnection::new()
            .await
            .expect("Failed to connect to the database"),
    );

    let client = db.get_client();
    let app = actix_web::test::init_service(
        App::new()
            .app_data(Data::new(client.clone()))
            .service(users_routes()),
    )
    .await;

    let payload = json!({
        "email": "unit_test@example.com",
        "password": "password123"
    });

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    // assert!(resp.status().is_success());

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["email"], "unit_test@example.com");
    assert_eq!(result["password"], "password123");
}

#[tokio::test]
pub async fn test_create_user_integration() {
    let client = reqwest::Client::new();

    let payload = CreateUserRequest {
        name: String::from("integration_test@example.com"),
        email: String::from("integration_test@example.com"),
        password: String::from("password123"),
        admin: true,
    };

    #[allow(unused_variables)]
    let response = client
        .post("http://127.0.0.1:8080/users")
        .form(&payload)
        .send()
        .await
        .expect("Failed to send request");

    // assert!(response.status().is_success());

    // let result: serde_json::Value = response.json().await.expect("Failed to parse response");
    // assert_eq!(result["email"], "integration_test@example.com");
    // assert_eq!(result["password"], "password123");
}
