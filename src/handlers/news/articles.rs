use crate::models::news::article::{
    CreateNewsArticleRequest, NewsArticle, UpdateNewsArticleRequest, UpdateNewsArticleStatusRequest,
};
use crate::models::news::NewsId;
use crate::utils::admin::verify_admin;
use crate::utils::auth::verify_ownership;
use crate::utils::jwt::verify_jwt;
use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

const TABLE: &str = "news_articles";
const OWNER_ID: &str = "writer_id";

pub async fn create_article(
    client: Data<Arc<Mutex<Client>>>,
    article: Json<CreateNewsArticleRequest>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for creating a news article");
    match verify_admin(&client, &req).await {
        // passes the news_role == 'writer' verification
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => warn!("Admin verification failed"),
        // needs to make the news_role == 'writer' verification
        _ => error!("Error verifying admin"),
    };

    debug!("Verifying news_role for writing a new article");
    let writer_id = verify_jwt(&req).expect("oi");
    // needs to see if user has news_role = writer
    debug!("Inserting new {} into the database", TABLE);
    let query: String = format!(
            "INSERT INTO {} (title, content, summary, writer_id, status) VALUES ($1, $2, $3, $4, $5) RETURNING id",
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
                &article.status,
            ],
        )
        .await
    {
        Ok(row) => {
            let id: NewsId = row.get("id");
            info!("Successfully created news article with id={}", id);
            HttpResponse::Created().json(id)
        }
        Err(err) => {
            error!("Failed to create news article: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to create news article")
        }
    }
}

pub async fn read_article(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
) -> impl Responder {
    debug!("Querying news article with id={}", article_id);
    let query: String = format!("SELECT * FROM {} WHERE id = $1", TABLE);
    match client.lock().await.query_opt(&query, &[&*article_id]).await {
        Ok(Some(row)) => {
            let article: NewsArticle = NewsArticle::from_row(row);
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

pub async fn read_articles(client: Data<Arc<Mutex<Client>>>) -> impl Responder {
    debug!("Querying all news articles from the database");
    let query: String = format!("SELECT * FROM {};", TABLE);
    match client.lock().await.query(&query, &[]).await {
        Ok(rows) => {
            let articles: Vec<NewsArticle> = rows
                .into_iter()
                .map(|row| NewsArticle::from_row(row))
                .collect();
            info!("Successfully retrieved all news articles");
            HttpResponse::Ok().json(articles)
        }
        Err(err) => {
            error!("Failed to retrieve news articles: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to fetch news articles")
        }
    }
}

pub async fn update_article(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    article: Json<UpdateNewsArticleRequest>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for updating a article");
    match verify_admin(&client, &req).await {
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => {
            warn!("User is not admin");
            match verify_ownership(&client, &req, TABLE, OWNER_ID).await {
                Ok(_) => info!("User owns the article, thus can update it"),
                Err(_) => {
                    return HttpResponse::Unauthorized()
                        .body("You cant edit someone's else article")
                }
            }
        }
        _ => error!("Error verifying admin"),
    }

    debug!("Updating article with id={}", article_id);
    let query = format!(
        "
        UPDATE {} SET
        title = $1, content = $2, summary = $3,
        updated_at = CURRENT_TIMESTAMP
        WHERE id = $4;",
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
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for deleting a article");
    match verify_admin(&client, &req).await {
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => {
            warn!("User is not admin");
            match verify_ownership(&client, &req, TABLE, OWNER_ID).await {
                Ok(_) => info!("User owns the article, thus can update it"),
                Err(_) => {
                    return HttpResponse::Unauthorized()
                        .body("You cant edit someone's else article")
                }
            }
        }
        _ => error!("Error verifying admin"),
    }

    debug!("Deleting article with id={}", article_id);
    let query: String = format!("DELETE FROM {} WHERE id = $1", TABLE);
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

pub async fn update_article_status(
    client: Data<Arc<Mutex<Client>>>,
    article_id: Path<NewsId>,
    article: Json<UpdateNewsArticleStatusRequest>,
    req: HttpRequest,
) -> impl Responder {
    // test if user is either:
    // role == admin
    // news_role == writer(owner) or news_role == editor or news_role == admin
    debug!("Verifying admin privileges for updating a article's status");
    match verify_admin(&client, &req).await {
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => {
            warn!("User is not admin");
            match verify_ownership(&client, &req, TABLE, OWNER_ID).await {
                Ok(_) => info!("User owns the article, thus can update it"),
                Err(_) => {
                    return HttpResponse::Unauthorized()
                        .body("You cant edit someone's else article")
                }
            }
        }
        _ => error!("Error verifying admin"),
    }

    debug!("Updating article's status with id={}", article_id);
    let update_string = if article.status == "published" {
        "published_at = CURRENT_TIMESTAMP,"
    } else {
        ""
    };
    let query = format!(
        "UPDATE {} SET status = $1, {} updated_at = CURRENT_TIMESTAMP
        WHERE id = $2;",
        TABLE, update_string
    );
    match client
        .lock()
        .await
        .execute(&query, &[&article.status, &*article_id])
        .await
    {
        Ok(rows_updated) if rows_updated > 0 => {
            info!("Successfully updated article with id={}", article_id);
            HttpResponse::Ok().body(format!("Article status updated to {}", article.status))
        }
        Ok(_) => {
            warn!("No article found with id={}", article_id);
            HttpResponse::NotFound().body(format!("Article '{}' not found", article_id))
        }
        Err(err) => {
            error!("Failed to update article with id={}: {:?}", article_id, err);
            HttpResponse::InternalServerError().body("Failed to update article's status")
        }
    }
}
