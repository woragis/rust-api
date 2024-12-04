use log::debug;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_news_articles_table(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating news articles table");
    // let create_enum = "CREATE TYPE news_articles_status AS ENUM ('draft', 'published', 'archived');";
    let create_table = "
        CREATE TABLE IF NOT EXISTS news_articles (
        id BIGSERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        content TEXT NOT NULL,
        summary VARCHAR(500),
        writer_id BIGINT REFERENCES users(id),
        -- category_id INT REFERENCES categories(category_id),
        status VARCHAR(9),
        published_at TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT check_news_article_status CHECK (status IN ('draft', 'published', 'archived'))
    );";
    // client.lock().await.execute(create_enum, &[]).await?;
    client.lock().await.execute(create_table, &[]).await?;
    Ok(())
}
