use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::db;
use db::connection::DbConnection;
use db::operations::users::{create_user, delete_user, read_user, read_users, update_user};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    name: String,
    email: String,
}

pub async fn create_user_handler(
    db: web::Data<DbConnection>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    let client = db.get_client();
    if let Err(err) = create_user(client, &user.name, &user.email).await {
        return HttpResponse::InternalServerError().body(format!("Error: {}", err));
    }
    HttpResponse::Created().finish()
}

pub async fn read_user_handler(
    db: web::Data<DbConnection>,
    user_id: web::Path<u32>,
) -> impl Responder {
    let client = db.get_client();
    match read_user(client, user_id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn read_users_handler(db: web::Data<DbConnection>) -> impl Responder {
    let client = db.get_client();
    match read_users(client).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn update_user_handler(
    db: web::Data<DbConnection>,
    user_id: web::Path<u32>,
    user: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let client = db.get_client();
    if let Err(err) = update_user(client, user_id.into_inner(), &user.name, &user.email).await {
        return HttpResponse::InternalServerError().body(format!("Error: {}", err));
    }
    HttpResponse::Ok().finish()
}

pub async fn delete_user_handler(
    db: web::Data<DbConnection>,
    user_id: web::Path<u32>,
) -> impl Responder {
    let client = db.get_client();
    if let Err(err) = delete_user(client, user_id.into_inner()).await {
        return HttpResponse::InternalServerError().body(format!("Error: {}", err));
    }
    HttpResponse::Ok().finish()
}
