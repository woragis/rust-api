mod db;
mod routes;
mod api;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use db::connection::DbConnection;
use api::users::{create_user_handler, read_users_handler, read_user_handler, update_user_handler, delete_user_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "host=localhost user=postgres password=yourpassword dbname=rust_api";
    let db=
    Arc::new(DbConnection::new(db_url).await.expect("Failed to connect to the database"),);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Share DB connection
            .route("/users", web::post().to(create_user_handler))
            .route("/users", web::get().to(read_users_handler))
            .route("/users/{id}", web::get().to(read_user_handler))
            .route("/users/{id}", web::put().to(update_user_handler))
            .route("/users/{id}", web::delete().to(delete_user_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
