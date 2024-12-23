use std::sync::Arc;

use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{
    db::tables::news::COMMENTS_TABLE,
    models::{
        news::{
            comment::{Comment, CreateComment, DeleteComment, EditComment},
            NewsId,
        },
        user::UserId,
    },
    utils::jwt::verify_jwt,
};

pub async fn read_comments(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
) -> impl Responder {
    let stmt: String = format!("SELECT * FROM {} WHERE article_id = $1;", COMMENTS_TABLE);

    match client.lock().await.query(&stmt, &[&*article_id]).await {
        Ok(rows) => {
            let comments: Vec<Comment> =
                rows.into_iter().map(|row| Comment::from_row(row)).collect();
            HttpResponse::Ok().json(comments)
        }
        _ => HttpResponse::InternalServerError().body("hey"),
    }
}

pub async fn create_comment(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    comment: Json<CreateComment>,
    req: HttpRequest,
) -> impl Responder {
    let user_id: UserId = verify_jwt(&req).expect("oi");

    let stmt: String = format!(
        "INSERT INTO {} (article_id, reader_id, content) VALUES ($1, $2, $3);",
        COMMENTS_TABLE
    );

    match client
        .lock()
        .await
        .query_one(&stmt, &[&*article_id, &user_id, &comment.content])
        .await
    {
        Ok(_) => HttpResponse::Created(),
        _ => HttpResponse::InternalServerError(),
    }
}

pub async fn edit_comment(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    comment: Json<EditComment>,
    req: HttpRequest,
) -> impl Responder {
    let user_id: UserId = verify_jwt(&req).expect("oi");

    let stmt: String = format!(
        "INSERT INTO {} (article_id, reader_id, content) VALUES ($1, $2, $3);",
        COMMENTS_TABLE
    );

    match client
        .lock()
        .await
        .query_one(&stmt, &[&*article_id, &user_id, &comment.content])
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        _ => HttpResponse::InternalServerError(),
    }
}

pub async fn delete_comment(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    comment: Json<DeleteComment>,
    req: HttpRequest,
) -> impl Responder {
    let user_id: UserId = verify_jwt(&req).expect("oi");

    let stmt: String = format!(
        "INSERT INTO {} (article_id, reader_id, content) VALUES ($1, $2, $3);",
        COMMENTS_TABLE
    );

    match client
        .lock()
        .await
        .query_one(&stmt, &[&*article_id, &user_id, &comment.id])
        .await
    {
        Ok(_) => HttpResponse::Ok(),
        _ => HttpResponse::InternalServerError(),
    }
}
