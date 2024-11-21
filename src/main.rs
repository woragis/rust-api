mod db;
mod controllers;
mod routes;

use actix_web::{web, App, HttpServer};
use db::connection::DbConnection;
use routes::users::configure_users_routes;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "host=localhost user=postgres password=yourpassword dbname=rust_api";
    let db = Arc::new(
        DbConnection::new(db_url)
            .await
            .expect("Failed to connect to the database"),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Share DB connection
            .configure(configure_users_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
