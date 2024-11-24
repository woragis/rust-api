use crate::models::user::User;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    name: String,
    email: String,
    password: String,
}

pub async fn create_user(
    client: web::Data<Arc<Mutex<Client>>>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    println!("Creating User");
    let query = "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id";
    match client
        .lock()
        .await
        .query_one(query, &[&user.name, &user.email, &user.password])
        .await
    {
        Ok(row) => {
            let id: i32 = row.get("id");
            println!("Created User '{}'", id);
            HttpResponse::Created().json(User {
                id,
                name: user.name.clone(),
                email: user.email.clone(),
                password: user.password.clone(),
            })
        }
        Err(err) => {
            eprintln!("Failed to create user: {}", err);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}

pub async fn read_user(
    client: web::Data<Arc<Mutex<Client>>>,
    user_id: web::Path<i32>,
) -> impl Responder {
    println!("Reading User '{}'", user_id);
    let query = "SELECT id, name, email, password FROM users WHERE id = $1";
    match client.lock().await.query_one(query, &[&*user_id]).await {
        Ok(row) => {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password: row.get("password"),
            };
            println!("Read User '{}'", user.id);
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            eprintln!("User not found: {}", err);
            HttpResponse::NotFound().body("User not found")
        }
    }
}

pub async fn read_users(client: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    println!("Reading Users");
    let query = "SELECT id, name, email, password FROM users";
    match client.lock().await.query(query, &[]).await {
        Ok(rows) => {
            let users: Vec<User> = rows
                .into_iter()
                .map(|row| User {
                    id: row.get("id"),
                    name: row.get("name"),
                    email: row.get("email"),
                    password: row.get("password"),
                })
                .collect();
            println!("Read Users");
            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            eprintln!("Error fetching users: {}", err);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}

pub async fn update_user(
    client: web::Data<Arc<Mutex<Client>>>,
    user_id: web::Path<i32>,
    user: web::Json<UpdateUserRequest>,
) -> impl Responder {
    println!("Updating User '{}'", user_id);
    let query = "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4";
    match client
        .lock()
        .await
        .execute(query, &[&user.name, &user.email, &user.password, &*user_id])
        .await
    {
        Ok(rows_updated) if rows_updated > 0 => {
            println!("Updated User '{}'", user_id);
            HttpResponse::Ok().body("User updated")
        }
        Ok(_) => HttpResponse::NotFound().body(format!("User '{}' not found", user_id)),
        Err(err) => {
            eprintln!("Failed to update user: {}", err);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
    }
}

pub async fn delete_user(
    client: web::Data<Arc<Mutex<Client>>>,
    user_id: web::Path<i32>,
) -> impl Responder {
    println!("Deleting User '{}'", user_id);
    let query = "DELETE FROM users WHERE id = $1";
    match client.lock().await.execute(query, &[&*user_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            println!("Updated User '{}'", user_id);
            HttpResponse::Ok().body("User deleted")
        }
        Ok(_) => HttpResponse::NotFound().body(format!("User '{}' not found", user_id)),
        Err(err) => {
            eprintln!("Failed to delete user: {}", err);
            HttpResponse::InternalServerError().body("Failed to delete user")
        }
    }
}
