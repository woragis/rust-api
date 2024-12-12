use std::sync::Arc;

use actix_web::{
    web::{Data, Path},
    HttpRequest, HttpResponse, Responder,
};
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{db::tables::news::LIKES_TABLE, models::news::NewsId, utils::jwt::verify_jwt};

pub async fn get_articles_likes(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
) -> impl Responder {
    let query = format!("SELECT * FROM {} WHERE article_id = $1;", LIKES_TABLE);

    match client.lock().await.query(&query, &[&*article_id]).await {
        Ok(_) => {
            HttpResponse::Ok().body("oi")
        }
        _ => HttpResponse::InternalServerError().body("hey"),
    }
}

pub async fn like_article(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = verify_jwt(&req).expect("oi");

    let query = format!(
        "INSERT INTO {} (article_id, reader_id, content) VALUES ($1, $2, $3);",
        LIKES_TABLE
    );

    match client
        .lock()
        .await
        .query_one(&query, &[&*article_id, &user_id])
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        _ => HttpResponse::InternalServerError(),
    }
}

pub async fn get_comments_likes(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
) -> impl Responder {
    let query = format!("SELECT * FROM {} WHERE article_id = $1;", LIKES_TABLE);

    match client.lock().await.query(&query, &[&*article_id]).await {
        Ok(_) => {
            HttpResponse::Ok().body("oi")
        }
        _ => HttpResponse::InternalServerError().body("hey"),
    }
}

pub async fn like_comment(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = verify_jwt(&req).expect("oi");

    let query = format!(
        "INSERT INTO {} (article_id, reader_id, content) VALUES ($1, $2, $3);",
        LIKES_TABLE
    );

    match client
        .lock()
        .await
        .query_one(&query, &[&*article_id, &user_id])
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        _ => HttpResponse::InternalServerError(),
    }
}
