use log::debug;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, Error};

pub const USERS_TABLE: &str = "users";

pub async fn create_users_table(client: Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating users table");

    let stmt: String = format!(
        "
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        first_name VARCHAR(100) NOT NULL,
        last_name VARCHAR(100) NOT NULL,
        email VARCHAR(255) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        decrypted_password VARCHAR(255) NOT NULL,
        role VARCHAR(5) NOT NULL DEFAULT 'user',
        blog_role VARCHAR(6) DEFAULT 'reader',
        news_role VARCHAR(6) DEFAULT 'reader',
        store_role VARCHAR(6) DEFAULT 'buyer',
        youtube_role VARCHAR(8) DEFAULT 'user',
        fanfic_role VARCHAR(6) DEFAULT 'reader',
        profile_picture TEXT,
        bio TEXT,
        phone_number VARCHAR(20),
        is_verified BOOLEAN DEFAULT FALSE,
        last_login TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT check_role CHECK (role IN ('user', 'admin')),
        CONSTRAINT check_blog_role CHECK (blog_role IN ('reader', 'writer')),
        CONSTRAINT check_news_role CHECK (news_role IN ('reader', 'writer', 'editor', 'admin')),
        CONSTRAINT check_store_role CHECK (store_role IN ('buyer', 'seller')),
        CONSTRAINT check_youtube_role CHECK (youtube_role IN ('user', 'youtuber')),
        CONSTRAINT check_fanfic_role CHECK (fanfic_role IN ('reader', 'writer'))
    );",
        USERS_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}
