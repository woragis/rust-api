use crate::db::connection::DbConnection;
use crate::models::user::CreateUserRequest;
use crate::routes::users::users_routes;
use actix_web::{
    test::{self, TestRequest},
    web::Data,
    App,
};
use reqwest::{Client, Response};
use serde_json::json;
use std::sync::Arc;

#[actix_web::test]
pub async fn test_create_user_unit() {
    let db = Arc::new(
        DbConnection::new()
            .await
            .expect("Failed to connect to the database"),
    );

    let client = db.get_client();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(client.clone()))
            .service(users_routes()),
    )
    .await;

    let payload = json!({
        "email": "unit_test@example.com",
        "password": "password123"
    });

    let req = TestRequest::post()
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
    let client = Client::new();

    let payload = CreateUserRequest {
        first_name: String::from("integration_test@example.com"),
        last_name: String::from("integration_test@example.com"),
        email: String::from("integration_test@example.com"),
        password: String::from("password123"),
        role: String::from("user"),
    };

    let response = client
        .post("http://127.0.0.1:8080/users")
        .form(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    // let result: serde_json::Value = response.json().await.expect("Failed to parse response");
    // assert_eq!(result["email"], "integration_test@example.com");
    // assert_eq!(result["password"], "password123");
}
