use actix_web::{web::Data, App, HttpServer};
use std::sync::Arc;

mod config;
mod db;
mod handlers;
mod models;
mod routes;

use db::connection::DbConnection;
use db::tables::users::create_users_table;
use routes::users::users_scope;

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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(users_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
