use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use tokio_postgres::Client;
use crate::db::operations::{create_user, read_users, update_user, delete_user};

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    name: String,
}

// Create User
async fn create_user_handler(
    db: web::Data<Client>,
    req: web::Json<CreateUserRequest>,
) -> impl Responder {
    match create_user(db.get_ref(), &req.name).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Read Users
async fn read_users_handler(db: web::Data<Client>) -> impl Responder {
    match read_users(db.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Update User
async fn update_user_handler(
    db: web::Data<Client>,
    web::Path(id): web::Path<i32>,
    req: web::Json<UpdateUserRequest>,
) -> impl Responder {
    match update_user(db.get_ref(), id, &req.name).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Delete User
async fn delete_user_handler(db: web::Data<Client>, web::Path(id): web::Path<i32>) -> impl Responder {
    match delete_user(db.get_ref(), id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Configure Routes
pub fn config_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user_handler))
            .route("", web::get().to(read_users_handler))
            .route("/{id}", web::put().to(update_user_handler))
            .route("/{id}", web::delete().to(delete_user_handler)),
    );
}
