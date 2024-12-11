use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{
    models::news::{comment::PostComment, NewsId},
    utils::jwt::verify_jwt,
};

const TABLE: &str = "news_comments";

pub async fn create_comment(
    client: web::Data<Arc<Mutex<Client>>>,
    article_id: web::Path<NewsId>,
    comment: web::Json<PostComment>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = verify_jwt(&req).expect("oi");
    let query = format!(
        "INSERT INTO {} (article_id, reader_id, content) VALUES ($1, $2, $3)",
        TABLE
    );
    match client
        .lock()
        .await
        .query_one(&query, &[&*article_id, &user_id, &comment.content])
        .await
    {
        Ok(_) => HttpResponse::Created().body("Created new comment"),
        _ => HttpResponse::InternalServerError().body("Error creating comment"),
    }
}
