use std::env;

pub fn get_db_string() -> String {
    // let host = "localhost";
    // let user = "postgres";
    // let password = "yourpassword";
    // let database = "rust_api";
    let host = env::var("DATABASE_HOST").expect("Failet to get DATABASE_HOST environment variable");
    let user= env::var("DATABASE_USER").expect("Failet to get DATABASE_USER environment variable");
    let password= env::var("DATABASE_PASSWORD").expect("Failet to get DATABASE_PASSWORD environment variable");
    let database= env::var("DATABASE_NAME").expect("Failet to get DATABASE_NAME environment variable");

    format!(
        "host={} user={} password={} dbname={}",
        host, user, password, database
    )
}
