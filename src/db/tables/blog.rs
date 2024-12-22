use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, Error};

use crate::db::tables::users::USERS_TABLE;

pub const POSTS_TABLE: &str = "blog_posts";
pub const SUBSCRIPTION_TABLE: &str = "blog_subscriptions";

pub async fn create_blog_tables(client: Arc<Mutex<Client>>) -> () {
    debug!("Creating blog tables");
    match create_posts_table(&client).await {
        Ok(_) => info!("Table '{}' created", POSTS_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", POSTS_TABLE, err),
    }
    match create_subscriptions_table(&client).await {
        Ok(_) => info!("Table '{}' created", SUBSCRIPTION_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", SUBSCRIPTION_TABLE, err),
    }
}

async fn create_posts_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating products table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        body TEXT NOT NULL,
        author_id BIGINT NOT NULL REFERENCES {}(id),
        visibility VARCHAR(7) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT check_blog_post_visibility CHECK (visibility IN ('hidden', 'visible', 'private'))
    );", POSTS_TABLE, USERS_TABLE);

    client
        .lock()
        .await
        .execute(&stmt, &[])
        .await?;

    Ok(())
}


async fn create_subscriptions_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating products table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        user_id BIGINT REFERENCES {}(id),
        author_id BIGINT REFERENCES {}(id),
        subscribed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );", SUBSCRIPTION_TABLE, USERS_TABLE, USERS_TABLE);

    client
        .lock()
        .await
        .execute(&stmt, &[])
        .await?;

    Ok(())
}

