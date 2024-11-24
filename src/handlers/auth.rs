use crate::models::auth::{LoginRequest, RegisterRequest};
use crate::models::user::User;
use crate::utils::bcrypt::{hash_password, verify_password};
use crate::utils::jwt::create_jwt;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn register(
    client: web::Data<Arc<Mutex<Client>>>,
    form: web::Json<RegisterRequest>,
) -> impl Responder {
    println!("Encrypting Password");
    let hashed_password = hash_password(&form.password);
    println!("Registering User");
    let query = "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id";
    match client
        .lock()
        .await
        .query_one(query, &[&form.name, &form.email, &hashed_password])
        .await
    {
        Ok(row) => {
            let id: u32 = row.get("id");
            println!("Registered User '{}'", id);
            HttpResponse::Created().json(User {
                id,
                name: form.name.clone(),
                email: form.email.clone(),
                password: form.password.clone(),
            })
        }
        Err(err) => {
            eprintln!("Failed to register user: {}", err);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}

pub async fn login(
    client: web::Data<Arc<Mutex<Client>>>,
    form: web::Json<LoginRequest>,
) -> impl Responder {
    let query = "SELECT * FROM users WHERE email = $1";
    match client.lock().await.query_opt(query, &[&form.email]).await {
        Ok(Some(row)) => {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password: row.get("password"),
            };
            println!("Found User '{}'", user.id);
            if verify_password(&user.password, &form.password) {
                println!("User '{}' - Password wsa correct", user.id);
                let token = create_jwt(&user, "oibanana");
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().body("Erro interno"),
    }
}
