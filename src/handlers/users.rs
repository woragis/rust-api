use crate::{
    models::user::{CreateUserRequest, UpdateUserRequest, User},
    utils::admin::verify_admin,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_user(
    client: web::Data<Arc<Mutex<Client>>>,
    user: web::Json<CreateUserRequest>,
    req: HttpRequest,
) -> impl Responder {
    println!("Testing if user is admin");
    match verify_admin(&client, &req).await {
        Ok(_) => println!("User is admin"),
        Err(_) => println!("User was not admin"),
    };
    println!("Creating User");
    let query = "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id";
    match client
        .lock()
        .await
        .query_one(query, &[&user.name, &user.email, &user.password])
        .await
    {
        Ok(row) => {
            let id = row.get("id");
            println!("Created User '{}'", id);
            HttpResponse::Created().json(User {
                id,
                name: user.name.clone(),
                email: user.email.clone(),
                password: user.password.clone(),
                admin: user.admin,
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
    req: HttpRequest,
) -> impl Responder {
    println!("Testing if user is admin");
    match verify_admin(&client, &req).await {
        Ok(_) => println!("User is admin"),
        Err(_) => println!("User was not admin"),
    };
    println!("Reading User '{}'", user_id);
    let query = "SELECT * FROM users WHERE id = $1";
    match client.lock().await.query_one(query, &[&*user_id]).await {
        Ok(row) => {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password: row.get("password"),
                admin: row.get("admin"),
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

pub async fn read_users(client: web::Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
) -> impl Responder {
    println!("Testing if user is admin");
    match verify_admin(&client, &req).await {
        Ok(_) => println!("User is admin"),
        Err(_) => println!("User was not admin"),
    };
    println!("Reading Users");
    let query = "SELECT * FROM users";
    match client.lock().await.query(query, &[]).await {
        Ok(rows) => {
            let users: Vec<User> = rows
                .into_iter()
                .map(|row| User {
                    id: row.get("id"),
                    name: row.get("name"),
                    email: row.get("email"),
                    password: row.get("password"),
                    admin: row.get("admin"),
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
    req: HttpRequest,
) -> impl Responder {
    println!("Testing if user is admin");
    match verify_admin(&client, &req).await {
        Ok(_) => println!("User is admin"),
        Err(_) => println!("User was not admin"),
    };
    println!("Updating User '{}'", user_id);
    let query = "UPDATE users SET name = $1, email = $2, password = $3, admin = $4 WHERE id = $5";
    match client
        .lock()
        .await
        .execute(
            query,
            &[
                &user.name,
                &user.email,
                &user.password,
                &user.admin,
                &*user_id,
            ],
        )
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
    req: HttpRequest,
) -> impl Responder {
    println!("Testing if user is admin");
    match verify_admin(&client, &req).await {
        Ok(_) => println!("User is admin"),
        Err(_) => println!("User was not admin"),
    };
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
