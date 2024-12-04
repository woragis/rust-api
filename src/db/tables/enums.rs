use log::{debug, error, info};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub struct RoleEnum {
    name: String,
    query: String,
    info: String,
    error: String,
}

pub async fn create_enum_types(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating enums");
    let create_role_enum = RoleEnum {
        name: "User role".to_string(),
        query: "CREATE TYPE role_enum AS ENUM ('user', 'admin');".to_string(),
        info: "Created user Role".to_string(),
        error: "Error creating user role".to_string(),
    };
    let create_blog_enum = RoleEnum {
        query: "CREATE TYPE blog_role_enum AS ENUM ('reader', 'writer');".to_string(),
        name: "User role".to_string(),
        info: "Created user Role".to_string(),
        error: "Error creating user role".to_string(),
    };
    let create_news_enum = RoleEnum {
        name: "News role".to_string(),
        query: "CREATE TYPE news_role_enum AS ENUM ('reader', 'writer', 'editor', 'admin');"
            .to_string(),
        info: "Created news role".to_string(),
        error: "Error creating news role".to_string(),
    };
    let create_store_enum = RoleEnum {
        name: "Store role".to_string(),
        query: "CREATE TYPE store_role_enum AS ENUM ('buyer', 'seller');".to_string(),
        info: "Created store role".to_string(),
        error: "Error creating store role".to_string(),
    };
    let create_youtube_enum = RoleEnum {
        name: "Youtube role".to_string(),
        query: "CREATE TYPE youtube_role_enum AS ENUM ('user', 'youtuber');".to_string(),
        info: "Created youtube role".to_string(),
        error: "Error creating youtube role".to_string(),
    };
    let create_fanfic_enum = RoleEnum {
        name: "Fanfic role".to_string(),
        query: "CREATE TYPE fanfic_role_enum AS ENUM ('reader', 'writer');".to_string(),
        info: "Created fanfic role".to_string(),
        error: "Error creating fanfic role".to_string(),
    };
    let enums: [RoleEnum; 6] = [
        create_role_enum,
        create_blog_enum,
        create_news_enum,
        create_store_enum,
        create_youtube_enum,
        create_fanfic_enum,
    ];

    for enum_type in enums {
        create_enum(&client, enum_type).await;
    }
    Ok(())
}

async fn create_enum(client: &Arc<Mutex<Client>>, query: RoleEnum) -> () {
    debug!("{}", query.name);
    match client.lock().await.execute(&query.query, &[]).await {
        Ok(_) => info!("{}", query.info),
        Err(_) => error!("{}", query.error),
    }
    ()
}
