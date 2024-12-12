use log::{debug, error, info};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub const ARTICLES_TABLE: &str = "news_articles";
pub const COMMENTS_TABLE: &str = "news_comments";
pub const LIKES_TABLE: &str = "news_articles";
pub const VIEWS_TABLE: &str = "news_views";
pub const TAGS_TABLE: &str = "news_tags";

pub async fn create_news_tables(client: Arc<Mutex<Client>>) -> () {
    match create_news_articles_table(&client).await {
        Ok(_) => info!("Table 'news_articles' created"),
        _ => error!("Table 'news_articles' not created"),
    }
    match create_news_comments_table(&client).await {
        Ok(_) => info!("Table 'news_comments' created"),
        _ => error!("Table 'news_comments' not created"),
    };
    match create_news_likes_table(&client).await {
        Ok(_) => info!("Table 'news_likes' created"),
        _ => error!("Table 'news_likes' not created"),
    };
    match create_news_views_table(&client).await {
        Ok(_) => info!("Table 'news_views' created"),
        _ => error!("Table 'news_views' not created"),
    };

    ()
}

async fn create_news_articles_table(client: &Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating news articles table");

    let create_table = "
        CREATE TABLE IF NOT EXISTS news_articles (
        id BIGSERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        content TEXT NOT NULL,
        summary VARCHAR(500),
        writer_id BIGINT REFERENCES users(id),
        -- category_id BIGINT REFERENCES categories(category_id),
        status VARCHAR(9),
        published_at TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT check_news_article_status CHECK (status IN ('draft', 'published', 'archived'))
    );";

    client.lock().await.execute(create_table, &[]).await?;

    Ok(())
}

async fn create_news_comments_table(client: &Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating news comments table");

    let create_table = "
        CREATE TABLE IF NOT EXISTS news_comments (
        id BIGSERIAL PRIMARY KEY,
        article_id BIGINT NOT NULL REFERENCES news_articles(id),
        reader_id BIGINT NOT NULL REFERENCES users(id),
        content TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );";

    client.lock().await.execute(create_table, &[]).await?;

    Ok(())
}

async fn create_news_likes_table(client: &Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating news likes table");

    let create_table = "
        CREATE TABLE IF NOT EXISTS news_likes (
        id BIGSERIAL PRIMARY KEY,
        article_id BIGINT REFERENCES news_articles(id),
        reader_id BIGINT REFERENCES users(id),
        UNIQUE (article_id, reader_id)
    );";

    client.lock().await.execute(create_table, &[]).await?;

    Ok(())
}

async fn create_news_views_table(client: &Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating news views table");

    let create_table = "
        CREATE TABLE IF NOT EXISTS news_views (
        id BIGSERIAL PRIMARY KEY,
        article_id BIGINT REFERENCES news_articles(id),
        views BIGINT
    );";

    client.lock().await.execute(create_table, &[]).await?;

    Ok(())
}
