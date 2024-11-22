pub fn get_db_string() -> String {
    let host = "localhost";
    let user = "postgres";
    let password = "yourpassword";
    let database = "rust_api";

    format!(
        "host={} user={} password={} dbname={}",
        host, user, password, database
    )
}
