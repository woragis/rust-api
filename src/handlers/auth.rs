use crate::models::auth::{LoginRequest, RegisterRequest, UpdateProfileRequest};
use crate::models::user::User;
use crate::utils::bcrypt::{hash_password, verify_password};
use crate::utils::jwt::{create_jwt, verify_jwt};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
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
    let query =
        "INSERT INTO users (first_name, last_name, email, password, role) VALUES ($1, $2, $3, $4, $5) RETURNING id;";
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
                &form.role,
            ],
        )
        .await
    {
        Ok(row) => {
            let id = row.get("id");
            println!("Registered User '{}'", id);
            let token = create_jwt(id, form.email.clone());
            HttpResponse::Created().json(token)
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
    let query = "SELECT id, email, password FROM users WHERE email = $1";
    match client.lock().await.query_opt(query, &[&form.email]).await {
        Ok(Some(row)) => {
            let user_id = row.get("id");
            let email = row.get("email");
            let password = row.get("password");
            println!("Found User '{}'", user_id);
            if verify_password(password, &form.password) {
                println!("User '{}' - Logged in", user_id);
                let token = create_jwt(user_id, email);
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().body("Erro interno"),
    }
}

pub async fn read_profile(
    client: web::Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
) -> impl Responder {
    match verify_jwt(&req) {
        Ok(user_id) => {
            let query = "SELECT * FROM users WHERE id = $1;";
            match client.lock().await.query_opt(query, &[&user_id]).await {
                Ok(Some(row)) => {
                    let user = User::from_row(row);
                    HttpResponse::Ok().json(user)
                }
                Ok(None) => HttpResponse::NotFound().body("User not found"),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("User profile not found {}", err)),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error in profile"),
    }
}

pub async fn update_profile(
    client: web::Data<Arc<Mutex<Client>>>,
    form: web::Json<UpdateProfileRequest>,
    req: HttpRequest,
) -> impl Responder {
    match verify_jwt(&req) {
        Ok(user_id) => {
            let query = "UPDATE users SET
            first_name = $1, last_name = $2, email = $3, password = $4,
            profile_picture = $5, phone_number = $6, is_verified = $7, last_login = $8,
            updated_at = CURRENT_TIMESTAMP WHERE id = $9;";
            match client
                .lock()
                .await
                .execute(
                    query,
                    &[
                        &form.first_name,
                        &form.last_name,
                        &form.email,
                        &form.password,
                        &form.profile_picture,
                        &form.phone_number,
                        &form.is_verified,
                        &form.last_login,
                        &user_id,
                    ],
                )
                .await
            {
                Ok(rows_updated) if rows_updated > 0 => HttpResponse::Ok().body("User updated"),
                Ok(_) => HttpResponse::NotFound().body(format!("User '{}' not found", user_id)),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("User profile not found {}", err)),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error in profile"),
    }
}

pub async fn delete_profile(
    client: web::Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
) -> impl Responder {
    match verify_jwt(&req) {
        Ok(user_id) => {
            let query = "DELETE FROM users WHERE id = $1;";
            match client.lock().await.execute(query, &[&user_id]).await {
                Ok(rows_deleted) if rows_deleted > 0 => HttpResponse::Ok().body("Deleted User"),
                Ok(_) => HttpResponse::NotFound().body(format!("User '{}' not found", user_id)),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("User profile not found {}", err)),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error in profile"),
    }
}
