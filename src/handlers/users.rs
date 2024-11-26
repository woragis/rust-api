use crate::{
    models::user::{CreateUserRequest, CreateUserResponse, UpdateUserRequest, User},
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
        Err(err) => {
            return HttpResponse::Unauthorized()
                .body(format!("You are not admin\nError: {:?}", err))
        }
    };
    println!("Creating User");
    let query = "INSERT INTO users (first_name, email, password) VALUES ($1, $2, $3) RETURNING id";
    match client
        .lock()
        .await
        .query_one(query, &[&user.first_name, &user.email, &user.password])
        .await
    {
        Ok(row) => {
            let id = row.get("id");
            println!("Created User '{}'", id);
            HttpResponse::Created().json(CreateUserResponse {
                id,
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                email: user.email.clone(),
                password: user.password.clone(),
                role: user.role.clone(),
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
        Err(err) => {
            return HttpResponse::Unauthorized()
                .body(format!("You are not admin\nError: {:?}", err))
        }
    };
    println!("Reading User '{}'", user_id);
    let query = "SELECT * FROM users WHERE id = $1";
    match client.lock().await.query_one(query, &[&*user_id]).await {
        Ok(row) => {
            let user = User::from_row(row);
            println!("Read User '{}'", user.id);
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            eprintln!("User not found: {}", err);
            HttpResponse::NotFound().body("User not found")
        }
    }
}

pub async fn read_users(client: web::Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    println!("Testing if user is admin");
    match verify_admin(&client, &req).await {
        Ok(_) => println!("User is admin"),
        Err(err) => {
            return HttpResponse::Unauthorized()
                .body(format!("You are not admin\nError: {:?}", err))
        }
    };
    println!("Reading Users");
    let query = "SELECT * FROM users";
    match client.lock().await.query(query, &[]).await {
        Ok(rows) => {
            let users: Vec<User> = rows.into_iter().map(|row| User::from_row(row)).collect();
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
        Err(err) => {
            return HttpResponse::Unauthorized()
                .body(format!("You are not admin\nError: {:?}", err))
        }
    };
    println!("Updating User '{}'", user_id);
    let query = "
        UPDATE users SET
        first_name = $1, last_name = $2, email = $3,
        password = $4, role = $5, profile_picture = $6, phone_number = $7,
        is_verified = $8, last_login = $9, updated_at = CURRENT_TIMESTAMP
        WHERE id = $10";
    match client
        .lock()
        .await
        .execute(
            query,
            &[
                &user.first_name,
                &user.last_name,
                &user.email,
                &user.password,
                &user.role,
                &user.profile_picture,
                &user.phone_number,
                &user.is_verified,
                &user.last_login,
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
        Err(err) => {
            return HttpResponse::Unauthorized()
                .body(format!("You are not admin\nError: {:?}", err))
        }
    };
    println!("Deleting User '{}'", user_id);
    let query = "DELETE FROM users WHERE id = $1";
    match client.lock().await.execute(query, &[&*user_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            println!("Deleted User '{}'", user_id);
            HttpResponse::Ok().body("User deleted")
        }
        Ok(_) => HttpResponse::NotFound().body(format!("User '{}' not found", user_id)),
        Err(err) => {
            eprintln!("Failed to delete user: {}", err);
            HttpResponse::InternalServerError().body("Failed to delete user")
        }
    }
}
