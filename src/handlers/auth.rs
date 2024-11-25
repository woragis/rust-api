use crate::models::auth::{LoginRequest, RegisterRequest, RegisterResponse};
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
        "INSERT INTO users (name, email, password, admin) VALUES ($1, $2, $3, $4) RETURNING id";
    match client
        .lock()
        .await
        .query_one(
            query,
            &[&form.name, &form.email, &hashed_password, &form.admin],
        )
        .await
    {
        Ok(row) => {
            let id = row.get("id");
            println!("Registered User '{}'", id);
            let user = User {
                id,
                name: form.name.clone(),
                email: form.email.clone(),
                password: form.password.clone(),
                admin: form.admin,
            };
            let token = create_jwt(&user);
            HttpResponse::Created().json(RegisterResponse {
                id,
                name: form.name.clone(),
                email: form.email.clone(),
                password: form.password.clone(),
                admin: form.admin,
                token,
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
                admin: row.get("admin"),
            };
            println!("Found User '{}'", user.id);
            if verify_password(&user.password, &form.password) {
                println!("User '{}' - Logged in", user.id);
                let token = create_jwt(&user);
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
                        name: row.get("name"),
                        email: row.get("email"),
                        password: row.get("password"),
                        admin: row.get("admin"),
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
