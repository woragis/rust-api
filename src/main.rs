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
use db::tables::news::create_news_tables;
use db::tables::password_manager::create_password_manager_tables;
use db::tables::store::create_store_tables;
use db::tables::users::create_users_table;
use log::{error, info};
use routes::auth::{auth_routes, profile_routes};
use routes::news::{news_articles_routes, news_tags_routes};
use routes::password_manager::password_manager_routes;
use routes::store::{orders_routes, products_routes};
use routes::users::users_routes;
use std::sync::Arc;
use utils::logger::setup_logger;
// use db::tables::enums::create_enum_types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    create_store_tables(client.clone()).await;
    create_news_tables(client.clone()).await;
    create_password_manager_tables(client.clone()).await;

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
            .service(password_manager_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
