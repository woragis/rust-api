mod db;
use db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db().await;
    println!("Program is running!");

    Ok(())
}
