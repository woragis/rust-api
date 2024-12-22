use std::sync::Arc;

use actix_web::{web::{Data, Json, Path}, HttpRequest, HttpResponse, Responder};
use log::info;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{db::tables::blog::POSTS_TABLE, models::blog::post::{BlogPost, CreateBlogPost, UpdateBlogPost}, shared::types::Id, utils::jwt::verify_jwt};

pub async fn create_post(client: Data<Arc<Mutex<Client>>>, post: Json<CreateBlogPost>, req: HttpRequest) -> impl Responder {
    let user_id = verify_jwt(&req).expect("hi");
    let stmt = format!("INSERT INTO {} (title, body, author_id, visibility) VALUES ($1, $2, $3, $4);", POSTS_TABLE);
    match client.lock().await.execute(&stmt, &[
        &post.title,
        &post.body,
        &user_id,
        &post.visibility
    ]).await {
        Ok(_) => HttpResponse::Created().body("created post"),
        Err(_) => HttpResponse::InternalServerError().body("Hi")
    }
}

pub async fn read_post(client: Data<Arc<Mutex<Client>>>, post_id: Path<Id>, req: HttpRequest) -> impl Responder {
    let user_id = verify_jwt(&req).expect("hi");
    user_id;
    let stmt = format!("SELECT * FROM {} WHERE id = $1;", POSTS_TABLE);
    match client.lock().await.query_opt(&stmt, &[
        &*post_id,
    ]).await {
        Ok(Some(row)) => {
            info!("Successfully found post '{}'", &*post_id);
            let post = BlogPost::from_row(row);
            HttpResponse::Ok().json(post)
        },
        Ok(None) => {
            HttpResponse::NotFound().body("Cound not find post")
        }
        Err(_) => HttpResponse::InternalServerError().body("Hi")
    }
}

pub async fn read_posts(client: Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    let user_id = verify_jwt(&req).expect("hi");
    user_id;
    let stmt = format!("SELECT * FROM {};", POSTS_TABLE);
    match client.lock().await.query(&stmt, &[]).await {
        Ok(rows) => {
            let posts: Vec<BlogPost> = rows.into_iter().map(|row| BlogPost::from_row(row)).collect();
            HttpResponse::Ok().json(posts)
        },
        Err(_) => HttpResponse::InternalServerError().body("Hi")
    }
}

pub async fn update_post(client: Data<Arc<Mutex<Client>>>, post_id: Path<Id>, post: Json<UpdateBlogPost>, req: HttpRequest) -> impl Responder {
    let user_id = verify_jwt(&req).expect("hi");
    let stmt = format!("UPDATE {} SET title = $1, body = $2, visibility = $3 WHERE id = $4 AND author_id = $5;", POSTS_TABLE);
    match client.lock().await.query_one(&stmt, &[
        &post.title,
        &post.body,
        &post.visibility,
        &*post_id,
        &user_id,
    ]).await {
        Ok(_) => HttpResponse::Ok().body("updated post"),
        Err(_) => HttpResponse::InternalServerError().body("Hi")
    }
}

pub async fn delete_post(client: Data<Arc<Mutex<Client>>>, post_id: Path<Id>, req: HttpRequest) -> impl Responder {
    let user_id = verify_jwt(&req).expect("hi");
    let stmt = format!("DELETE FROM {} WHERE id = $1 AND author_id = $2;", POSTS_TABLE);
    match client.lock().await.execute(&stmt, &[
        &*post_id,
        &user_id,
    ]).await {
        Ok(_) => HttpResponse::Ok().body("deleted post"),
        Err(_) => HttpResponse::InternalServerError().body("Hi")
    }
}