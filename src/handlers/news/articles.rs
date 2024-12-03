use crate::models::news::article::{
    CreateNewsArticleRequest, NewsArticle, UpdateNewsArticleRequest,
};
use crate::models::news::NewsId;
use crate::utils::admin::verify_admin;
use crate::utils::auth::verify_ownership;
use crate::utils::jwt::verify_jwt;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

const TABLE: &str = "news_articles";
const ID: &str = "id";

pub async fn create_article(
    client: web::Data<Arc<Mutex<Client>>>,
    article: web::Json<CreateNewsArticleRequest>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for creating a news article");
    match verify_admin(&client, &req).await {
        true => info!("Admin privileges verified"),
        false => warn!("Admin verification failed"),
    };

    if let Some(writer_id) = verify_jwt(&req) {
        debug!("Inserting new {} into the database", TABLE);
        let query = format!(
            "INSERT INTO {} (
        title, content, summary, writer_id) VALUES ($1, $2, $3, $4) RETURNING id",
            TABLE
        );
        match client
            .lock()
            .await
            .query_one(
                &query,
                &[
                    &article.title,
                    &article.content,
                    &article.summary,
                    &writer_id,
                ],
            )
            .await
        {
            Ok(row) => {
                let id: NewsId = row.get("id");
                info!("Successfully created news article with id={}", id);
                HttpResponse::Created().json(NewsArticle::from_row(row))
            }
            Err(err) => {
                error!("Failed to create news article: {:?}", err);
                HttpResponse::InternalServerError().body("Failed to create news article")
            }
        }
    } else {
        HttpResponse::InternalServerError().body("Failed to create news article")
    }
}

pub async fn read_article(
    client: web::Data<Arc<Mutex<Client>>>,
    article_id: web::Path<NewsId>,
) -> impl Responder {
    debug!("Querying news article with id={}", article_id);
    let query = format!("SELECT * FROM {} WHERE id = $1", TABLE);
    match client.lock().await.query_opt(&query, &[&*article_id]).await {
        Ok(Some(row)) => {
            let article = NewsArticle::from_row(row);
            info!("Successfully retrieved news article with id={}", article.id);
            HttpResponse::Ok().json(article)
        }
        Ok(None) => {
            warn!("No article found with id={}", article_id);
            HttpResponse::NotFound().body(format!("Article '{}' not found", article_id))
        }
        Err(err) => {
            error!(
                "Failed to retrieve article with id={}: {:?}",
                article_id, err
            );
            HttpResponse::NotFound().body("Article not found")
        }
    }
}

pub async fn read_articles(client: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    debug!("Querying all news articles from the database");
    let query = format!("SELECT * FROM {};", TABLE);
    match client.lock().await.query(&query, &[]).await {
        Ok(rows) => {
            let users: Vec<NewsArticle> = rows
                .into_iter()
                .map(|row| NewsArticle::from_row(row))
                .collect();
            info!("Successfully retrieved all news articles");
            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            error!("Failed to retrieve news articles: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to fetch news articles")
        }
    }
}

pub async fn update_article(
    client: web::Data<Arc<Mutex<Client>>>,
    article_id: web::Path<NewsId>,
    article: web::Json<UpdateNewsArticleRequest>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for updating a article");
    match verify_admin(&client, &req).await {
        true => info!("Admin privileges verified"),

        false => {
            warn!("Admin verification failed");
            match verify_ownership(&client, &req, TABLE, ID).await {
                Ok(_) => info!("User verified"),
                Err(_) => return HttpResponse::Unauthorized().body("Hey"),
            }
        }
    }

    debug!("Updating article with id={}", article_id);
    let query = format!(
        "
        UPDATE {} SET
        title = $1, content = $2, summary = $3,
        status = $4, updated_at = CURRENT_TIMESTAMP
        WHERE id = $5;",
        TABLE
    );
    match client
        .lock()
        .await
        .execute(
            &query,
            &[
                &article.title,
                &article.content,
                &article.summary,
                &article.status,
                &*article_id,
            ],
        )
        .await
    {
        Ok(rows_updated) if rows_updated > 0 => {
            info!("Successfully updated article with id={}", article_id);
            HttpResponse::Ok().body("Article updated")
        }
        Ok(_) => {
            warn!("No article found with id={}", article_id);
            HttpResponse::NotFound().body(format!("Article '{}' not found", article_id))
        }
        Err(err) => {
            error!("Failed to update article with id={}: {:?}", article_id, err);
            HttpResponse::InternalServerError().body("Failed to update article")
        }
    }
}

pub async fn delete_article(
    client: web::Data<Arc<Mutex<Client>>>,
    article_id: web::Path<NewsId>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for deleting a article");
    match verify_admin(&client, &req).await {
        true => info!("Admin privileges verified"),

        false => {
            warn!("Admin verification failed");
            match verify_ownership(&client, &req, TABLE, ID).await {
                Ok(_) => info!("User verified"),
                Err(_) => {
                    return HttpResponse::Unauthorized()
                        .body("You cant delete other people articles")
                }
            }
        }
    }

    debug!("Deleting article with id={}", article_id);
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    match client.lock().await.execute(&query, &[&*article_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            info!("Successfully deleted article with id={}", article_id);
            HttpResponse::Ok().body("Article deleted")
        }
        Ok(_) => {
            warn!("No article found with id={}", article_id);
            HttpResponse::NotFound().body(format!("Article '{}' not found", article_id))
        }
        Err(err) => {
            error!("Failed to delete article with id={}: {:?}", article_id, err);
            HttpResponse::InternalServerError().body("Failed to delete article")
        }
    }
}
