use crate::models::auth::{LoginRequest, RegisterRequest};
use crate::utils::bcrypt::{hash_password, verify_password};
use crate::utils::jwt::create_jwt;
use actix_web::{web, HttpResponse, Responder};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn register(
    client: web::Data<Arc<Mutex<Client>>>,
    form: web::Json<RegisterRequest>,
) -> impl Responder {
    debug!("Registering user");
    let hashed_password = hash_password(&form.password);
    let query =
        "INSERT INTO users (first_name, last_name, email, password, decrypted_password, role) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id;";
    match client
        .lock()
        .await
        .query_one(
            query,
            &[
                &form.first_name,
                &form.last_name,
                &form.email,
                &hashed_password,
                &form.password,
                &form.role,
            ],
        )
        .await
    {
        Ok(row) => {
            let id = row.get("id");
            info!("Successfully registered user with id={}", id);
            let token = create_jwt(id, form.email.clone());
            HttpResponse::Created().json(token)
        }
        Err(err) => {
            error!("Failed to register user: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}

pub async fn login(
    client: web::Data<Arc<Mutex<Client>>>,
    form: web::Json<LoginRequest>,
) -> impl Responder {
    debug!("Logging user");
    let query = "SELECT id, email, password FROM users WHERE email = $1";
    match client.lock().await.query_opt(query, &[&form.email]).await {
        Ok(Some(row)) => {
            let user_id = row.get("id");
            let email = row.get("email");
            let password = row.get("password");
            if verify_password(password, &form.password) {
                info!("Successfuly logged in user with id={}", user_id);
                let token = create_jwt(user_id, email);
                HttpResponse::Ok().json(token)
            } else {
                warn!("Failed to login - Invalid credentials");
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Ok(None) => {
            warn!("No user found with email={}", form.email);
            HttpResponse::Unauthorized().body("User not found")
        }
        Err(err) => {
            error!("Failed to login user with email={}: {:?}", form.email, err);
            HttpResponse::InternalServerError().body("Erro interno")
        }
    }
}
