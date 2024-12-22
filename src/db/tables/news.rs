use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, Error};

use crate::db::tables::users::USERS_TABLE;

pub const ARTICLES_TABLE: &str = "news_articles";
pub const COMMENTS_TABLE: &str = "news_comments";
pub const LIKES_TABLE: &str = "news_likes";
pub const VIEWS_TABLE: &str = "news_views";
pub const TAGS_TABLE: &str = "news_tags";
pub const ARTICLES_TAGS_TABLE: &str = "news_articles_tags";

pub async fn create_news_tables(client: Arc<Mutex<Client>>) -> () {
    match create_news_articles_table(&client).await {
        Ok(_) => info!("Table '{}' created", ARTICLES_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", ARTICLES_TABLE, err),
    }

    match create_news_comments_table(&client).await {
        Ok(_) => info!("Table '{}' created", COMMENTS_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", COMMENTS_TABLE, err),
    };

    match create_news_likes_table(&client).await {
        Ok(_) => info!("Table '{}' created", LIKES_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", LIKES_TABLE, err),
    };

    match create_news_views_table(&client).await {
        Ok(_) => info!("Table '{}' created", VIEWS_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", VIEWS_TABLE, err),
    };

    match create_news_tags_table(&client).await {
        Ok(_) => info!("Table '{}' created", TAGS_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", TAGS_TABLE, err),
    }

    match create_news_articles_tags_table(&client).await {
        Ok(_) => info!("Table '{}' created", ARTICLES_TAGS_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", ARTICLES_TAGS_TABLE, err),
    }

    ()
}

async fn create_news_articles_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating news articles table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        content TEXT NOT NULL,
        summary VARCHAR(500),
        writer_id BIGINT REFERENCES {}(id),
        -- category_id BIGINT REFERENCES categories(category_id),
        status VARCHAR(9),
        published_at TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT check_news_article_status CHECK (status IN ('draft', 'published', 'archived'))
    );",
        ARTICLES_TABLE, USERS_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}

async fn create_news_comments_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating news comments table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        article_id BIGINT NOT NULL REFERENCES {}(id),
        reader_id BIGINT NOT NULL REFERENCES {}(id),
        content TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );",
        COMMENTS_TABLE, ARTICLES_TABLE, USERS_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}

async fn create_news_likes_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating news likes table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        article_id BIGINT REFERENCES {}(id),
        reader_id BIGINT REFERENCES {}(id),
        UNIQUE (article_id, reader_id)
    );",
        LIKES_TABLE, ARTICLES_TABLE, USERS_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}

async fn create_news_views_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating news views table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        article_id BIGINT REFERENCES {}(id),
        views BIGINT
    );",
        VIEWS_TABLE, ARTICLES_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}

async fn create_news_tags_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating news tags table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        article_id BIGINT REFERENCES {}(id),
        views BIGINT
    );",
        TAGS_TABLE, ARTICLES_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}

async fn create_news_articles_tags_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating news articles tags table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        article_id BIGINT REFERENCES {}(id),
        tag_id BIGINT REFERENCES {}(id),
        PRIMARY KEY (article_id, tag_id)
    );",
        ARTICLES_TAGS_TABLE, ARTICLES_TABLE, TAGS_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}
