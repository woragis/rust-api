use log::debug;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_users_table(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating users table");
    let create_users_table_sql = "
        CREATE TABLE IF NOT EXISTS users (
        id BIGSERIAL PRIMARY KEY,
        first_name VARCHAR(100) NOT NULL,
        last_name VARCHAR(100) NOT NULL,
        email VARCHAR(255) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        decrypted_password VARCHAR(255) NOT NULL,
        role ENUM('user', 'admin') DEFAULT 'user',
        blog_role ENUM('reader', 'writer') DEFAULT 'reader',
        news_role ENUM('reader', 'writer', 'editor', 'admin') DEFAULT 'reader',
        store_role ENUM('buyer', 'seller') DEFAULT 'buyer',
        youtube_role ENUM('user', 'youtuber') DEFAULT 'user',
        fanfic_role ENUM('reader', 'writer') DEFAULT 'reader',
        profile_picture TEXT,
        bio TEXT,
        phone_number VARCHAR(20),
        is_verified BOOLEAN DEFAULT FALSE,
        last_login TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );";
    client
        .lock()
        .await
        .execute(create_users_table_sql, &[])
        .await?;
    Ok(())
}
