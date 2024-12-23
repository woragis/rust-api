use crate::db::tables::users::USERS_TABLE;
use crate::models::auth::{AuthResponse, AuthUser, LoginRequest, RegisterRequest};
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
    let stmt: String= format!("INSERT INTO {} (first_name, last_name, email, password, decrypted_password, role) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;", USERS_TABLE);
    match client
        .lock()
        .await
        .query_one(
            &stmt,
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
            let user: AuthUser = AuthUser::from_row(row);
            let user_id: UserId = user.id.clone();
            info!("Successfully registered user with id={}", user_id);
            let token: String = create_jwt(user_id, form.email.clone());
            let response: AuthResponse = AuthResponse { token, user };
            HttpResponse::Created().json(response)
        }
        Err(err) => {
            error!("Failed to register user: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}

pub async fn login(client: Data<Arc<Mutex<Client>>>, form: Json<LoginRequest>) -> impl Responder {
    debug!("Logging user");
    let stmt: String = format!("SELECT * FROM {} WHERE email = $1;", USERS_TABLE);
    match client.lock().await.query_opt(&stmt, &[&form.email]).await {
        Ok(Some(row)) => {
            let password: String = row.get("password");
            let user: AuthUser = AuthUser::from_row(row);
            let user_id: UserId = user.id.clone();
            let email: String = user.email.clone();
            if verify_password(&password, &form.password) {
                info!("Successfuly logged in user with id={}", user_id);
                let token: String = create_jwt(user_id, email);
                let response: AuthResponse = AuthResponse { token, user };
                HttpResponse::Ok().json(response)
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
