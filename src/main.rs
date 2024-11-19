use actix_web::{web, App, HttpServer, Responder};

async fn hello() -> impl Responder {
    "Hello World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Program is running!");
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
