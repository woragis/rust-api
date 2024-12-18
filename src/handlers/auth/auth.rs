use crate::models::auth::{LoginRequest, RegisterRequest};
use crate::models::user::UserId;
use crate::utils::bcrypt::{hash_password, verify_password};
use crate::utils::jwt::create_jwt;
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn register(
    client: Data<Arc<Mutex<Client>>>,
    form: Json<RegisterRequest>,
) -> impl Responder {
    debug!("Registering user");
    let hashed_password = hash_password(&form.password);
    let query: &str =
        "INSERT INTO users (first_name, last_name, email, password, decrypted_password, role) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;";
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
            // let user: User = User::from_row(row);
            let id: UserId = row.get("id");
            info!("Successfully registered user with id={}", id);
            let token: String = create_jwt(id, form.email.clone());
            HttpResponse::Created().json(token)
        }
        Err(err) => {
            error!("Failed to register user: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}

pub async fn login(client: Data<Arc<Mutex<Client>>>, form: Json<LoginRequest>) -> impl Responder {
    debug!("Logging user");
    let query = "SELECT id, email, password FROM users WHERE email = $1";
    match client.lock().await.query_opt(query, &[&form.email]).await {
        Ok(Some(row)) => {
            let user_id: UserId = row.get("id");
            let email: String = row.get("email");
            let password: String = row.get("password");
            if verify_password(&password, &form.password) {
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
