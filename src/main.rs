mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod shared;
mod tests;
mod utils;

use actix_web::{web::Data, App, HttpServer};
use db::connection::DbConnection;
use log::{error, info};
use handlers::password_manager::encryption::{decrypt, encrypt, generate_key};
// use db::tables::enums::create_enum_types;
use db::tables::news::create_news_articles_table;
use db::tables::orders::create_orders_table;
use db::tables::products::create_products_table;
use db::tables::users::create_users_table;
use routes::auth::{auth_routes, profile_routes};
use routes::news::{news_articles_routes, news_tags_routes};
use routes::store::{orders_routes, products_routes};
use routes::users::users_routes;
use utils::logger::setup_logger;
use std::sync::Arc;

#[actix_web::main]
async fn not_main() -> std::io::Result<()> {
    if let Err(err) = setup_logger() {
        eprintln!("Failed to initialize logger: {:?}", err);
        panic!("Failed to initialize logger: {:?}", err);
    }

    let db = Arc::new(
        DbConnection::new()
            .await
            .expect("Failed to connect to the database"),
    );

    let client = db.get_client();

    // match create_enum_types(client.clone()).await {Ok(_) => info!("Enums Types Created"),Err(err) => error!("Failed to create enum types: {:?}", err),}
    match create_users_table(client.clone()).await {
        Ok(_) => info!("Users Table Created"),
        Err(err) => error!("Failed to create users table: {:?}", err),
    }

    match create_products_table(client.clone()).await {
        Ok(_) => info!("Products Table Created"),
        Err(err) => error!("Failed to create products table: {:?}", err),
    }

    match create_orders_table(client.clone()).await {
        Ok(_) => info!("Orders Table Created"),
        Err(err) => error!("Failed to create orders table: {:?}", err),
    }

    match create_news_articles_table(client.clone()).await {
        Ok(_) => info!("News Articles Table Created"),
        Err(err) => error!("Failed to create news articles table: {:?}", err),
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(users_routes())
            .service(products_routes())
            .service(auth_routes())
            .service(profile_routes())
            .service(orders_routes())
            .service(news_articles_routes())
            .service(news_tags_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Example usage
fn main() {
    // Example usage
    let key = generate_key();
    let data = "Hello, world!";

    // Encrypt the data
    let (encrypted_data, iv) = encrypt(data, &key);
    println!("Encrypted: {}", encrypted_data);

    // Decrypt the data
    let decrypted_data = decrypt(&encrypted_data, &key, &iv.into());
    println!("Decrypted: {}", decrypted_data);
}
