use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_users_table(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    let create_users_table_sql = "
        CREATE TABLE IF NOT EXISTS users (
        id BIGSERIAL PRIMARY KEY,
        name TEXT,
        email TEXT UNIQUE NOT NULL,
        password TEXT NOT NULL,
        admin BOOL NOT NULL DEFAULT FALSE
    );";
    client
        .lock()
        .await
        .execute(create_users_table_sql, &[])
        .await?;
    Ok(())
}
