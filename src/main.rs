mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod tests;
mod utils;

use actix_web::{web::Data, App, HttpServer};
use db::connection::DbConnection;
use db::tables::orders::create_orders_table;
use db::tables::products::create_products_table;
use db::tables::users::create_users_table;
use routes::auth::{auth_routes, profile_routes};
use routes::orders::orders_routes;
use routes::products::products_routes;
use routes::users::users_routes;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(
        DbConnection::new()
            .await
            .expect("Failed to connect to the database"),
    );

    let client = db.get_client();
    println!("DB Client was successfully received");

    match create_users_table(client.clone()).await {
        Ok(_) => println!("Users Table Created"),
        Err(err) => eprintln!("Failed to create Users Table\nError: {}", err),
    }

    match create_products_table(client.clone()).await {
        Ok(_) => println!("Products Table Created"),
        Err(err) => eprintln!("Failed to create Products Table\nError: {}", err),
    }

    match create_orders_table(client.clone()).await {
        Ok(_) => println!("Orders Table Created"),
        Err(err) => eprintln!("Failed to create Orders Table\nError: {}", err),
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(users_routes())
            .service(products_routes())
            .service(auth_routes())
            .service(profile_routes())
            .service(orders_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
