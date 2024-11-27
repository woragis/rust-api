use crate::models::user::{CreateUserRequest, CreateUserResponse, UpdateUserRequest, User};
use crate::utils::admin::verify_admin;
use crate::utils::bcrypt::hash_password;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_user(
    client: web::Data<Arc<Mutex<Client>>>,
    user: web::Json<CreateUserRequest>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for creating a user");
    match verify_admin(&client, &req).await {
        Ok(_) => info!("Admin privileges verified"),
        Err(err) => {
            warn!("Admin verification failed: {:?}", err);
            return HttpResponse::Unauthorized().body("You are not admin");
        }
    };

    debug!("Inserting new user into the database");
    let hashed_password = hash_password(&user.password);
    let query = "INSERT INTO users (first_name, email, password, decrypted_password) VALUES ($1, $2, $3, $4) RETURNING id";
    match client
        .lock()
        .await
        .query_one(
            query,
            &[
                &user.first_name,
                &user.email,
                &hashed_password,
                &user.password,
            ],
        )
        .await
    {
        Ok(row) => {
            let id = row.get("id");
            info!("Successfully created user with id={}", id);
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
            error!("Failed to create user: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}

pub async fn read_user(
    client: web::Data<Arc<Mutex<Client>>>,
    user_id: web::Path<i32>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for reading a user");
    match verify_admin(&client, &req).await {
        Ok(_) => info!("Admin privileges verified"),
        Err(err) => {
            warn!("Admin verification failed: {:?}", err);
            return HttpResponse::Unauthorized().body("You are not admin");
        }
    };

    debug!("Querying user with id={}", user_id);
    let query = "SELECT * FROM users WHERE id = $1";
    match client.lock().await.query_opt(query, &[&*user_id]).await {
        Ok(Some(row)) => {
            let user = User::from_row(row);
            info!("Successfully retrieved user with id={}", user.id);
            HttpResponse::Ok().json(user)
        }
        Ok(None) => {
            warn!("No user found with id={}", user_id);
            HttpResponse::NotFound().body(format!("User '{}' not found", user_id))
        }
        Err(err) => {
            error!("Failed to retrieve user with id={}: {:?}", user_id, err);
            HttpResponse::NotFound().body("User not found")
        }
    }
}

pub async fn read_users(client: web::Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    debug!("Verifying admin privileges for reading all users");
    match verify_admin(&client, &req).await {
        Ok(_) => info!("Admin privileges verified"),
        Err(err) => {
            warn!("Admin verification failed: {:?}", err);
            return HttpResponse::Unauthorized().body("You are not admin");
        }
    };

    debug!("Querying all users from the database");
    let query = "SELECT * FROM users";
    match client.lock().await.query(query, &[]).await {
        Ok(rows) => {
            let users: Vec<User> = rows.into_iter().map(|row| User::from_row(row)).collect();
            info!("Successfully retrieved all users");
            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            error!("Failed to retrieve users: {:?}", err);
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
    debug!("Verifying admin privileges for updating a user");
    match verify_admin(&client, &req).await {
        Ok(_) => info!("Admin privileges verified"),
        Err(err) => {
            warn!("Admin verification failed: {:?}", err);
            return HttpResponse::Unauthorized().body("You are not admin");
        }
    };

    debug!("Updating user with id={}", user_id);
    let hashed_password = hash_password(&user.password);
    let query = "
        UPDATE users SET
        first_name = $1, last_name = $2, email = $3,
        password = $4, decrypted_password = $5, role = $6,
        blog_role = $7, store_role = $8, youtube_role = $9, fanfic_role = $10,
        profile_picture = $11, phone_number = $12,
        is_verified = $13, last_login = $14, updated_at = CURRENT_TIMESTAMP
        WHERE id = $15";
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
                &hashed_password,
                &user.role,
                &user.blog_role,
                &user.store_role,
                &user.youtube_role,
                &user.fanfic_role,
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
            info!("Successfully updated user with id={}", user_id);
            HttpResponse::Ok().body("User updated")
        }
        Ok(_) => {
            warn!("No user found with id={}", user_id);
            HttpResponse::NotFound().body(format!("User '{}' not found", user_id))
        }
        Err(err) => {
            error!("Failed to update user with id={}: {:?}", user_id, err);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
    }
}

pub async fn delete_user(
    client: web::Data<Arc<Mutex<Client>>>,
    user_id: web::Path<i32>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for deleting a user");
    match verify_admin(&client, &req).await {
        Ok(_) => info!("Admin privileges verified"),
        Err(err) => {
            warn!("Admin verification failed: {:?}", err);
            return HttpResponse::Unauthorized().body("You are not admin");
        }
    };

    debug!("Deleting user with id={}", user_id);
    let query = "DELETE FROM users WHERE id = $1";
    match client.lock().await.execute(query, &[&*user_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            info!("Successfully deleted user with id={}", user_id);
            HttpResponse::Ok().body("User deleted")
        }
        Ok(_) => {
            warn!("No user found with id={}", user_id);
            HttpResponse::NotFound().body(format!("User '{}' not found", user_id))
        }
        Err(err) => {
            error!("Failed to delete user with id={}: {:?}", user_id, err);
            HttpResponse::InternalServerError().body("Failed to delete user")
        }
    }
}
