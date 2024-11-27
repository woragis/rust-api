mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod tests;
mod utils;

use actix_web::{web::Data, App, HttpServer};
use chrono::Local;
use colored::*;
use db::connection::DbConnection;
use db::tables::orders::create_orders_table;
use db::tables::products::create_products_table;
use db::tables::users::create_users_table;
use fern::{log_file, Dispatch};
use log::{error, info};
use routes::auth::{auth_routes, profile_routes};
use routes::orders::orders_routes;
use routes::products::products_routes;
use routes::users::users_routes;
use std::sync::Arc;

fn setup_logger() -> Result<(), fern::InitError> {
    let file = log_file("log.log");
    // Configure fern logger with various logging outputs and formats
    Dispatch::new()
        .level(log::LevelFilter::Off)
        .level_for("api", log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] - {}",
                Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level().to_string().color(match record.level() {
                    log::Level::Error => "red",
                    log::Level::Warn => "yellow",
                    log::Level::Info => "green",
                    log::Level::Debug => "blue",
                    log::Level::Trace => "magenta",
                }),
                // record.target(),
                message
            ))
        })
        .chain(std::io::stdout()) // Log to standard output
        .chain(file?)
        .apply()
        .unwrap();

    Ok(())
}

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
