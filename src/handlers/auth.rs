use crate::models::auth::{LoginRequest, RegisterRequest};
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
        "INSERT INTO users (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING id;";
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

pub async fn profile(client: web::Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    match verify_jwt(&req) {
        Ok(user_id) => {
            let query = "SELECT * FROM users WHERE id = $1;";
            match client.lock().await.query_one(query, &[&user_id]).await {
                Ok(row) => {
                    let user = User {
                        id: row.get("id"),
                        first_name: row.get("first_name"),
                        last_name: row.get("last_name"),
                        email: row.get("email"),
                        password: row.get("password"),
                        role: row.get("role"),
                        profile_picture: row.get("profile_picture"),
                        phone_number: row.get("phone_number"),
                        is_verified: row.get("is_verified"),
                        last_login: row.get("last_login"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                    };
                    HttpResponse::Accepted().json(user)
                }
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("User profile not found {}", err)),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error in profile"),
    }
}
