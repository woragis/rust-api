use std::sync::Arc;

use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{
    models::news::{comment::{Comment, CreateComment, DeleteComment, EditComment}, NewsId},
    utils::jwt::verify_jwt,
};

const TABLE: &str = "news_likes";

pub async fn get_likes(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
) -> impl Responder {
    let query = format!(
        "SELECT * FROM {} WHERE article_id = $1;",
        TABLE
    );

    match client
        .lock()
        .await
        .query(&query, &[&*article_id])
        .await
    {
        Ok(rows) => {
            let comments: Vec<Comment> = rows.into_iter().map(|row| Comment::from_row(row)).collect();
            HttpResponse::Ok().json(comments)
        }
        _ => HttpResponse::InternalServerError().body("hey"),
    }
}

pub async fn like_comment(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    comment_id: Json<DeleteComment>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = verify_jwt(&req).expect("oi");

    let query = format!(
        "INSERT INTO {} (article_id, reader_id, content) VALUES ($1, $2, $3);",
        TABLE
    );

    match client
        .lock()
        .await
        .query_one(&query, &[&*article_id, &user_id, &comment_id.id])
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        _ => HttpResponse::InternalServerError(),
    }
}
