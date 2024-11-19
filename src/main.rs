mod db;
mod routes;

use actix_web::{web, App, HttpServer};
use db::connection::connect_db;
use routes::user_routes::config_user_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish database connection
    let client = connect_db()
        .await
        .expect("Failed to connect to the database");

    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone())) // Share DB connection
            .configure(config_user_routes) // Configure user routes
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
