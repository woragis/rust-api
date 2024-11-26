use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_users_table(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    let create_users_table_sql = "
        CREATE TABLE IF NOT EXISTS users (
        id BIGSERIAL PRIMARY KEY,
        first_name VARCHAR(100) NOT NULL,
        last_name VARCHAR(100) NOT NULL,
        email VARCHAR(255) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        role VARCHAR(50) DEFAULT 'user' CHECK (role in ('user', 'admin')),
        profile_picture TEXT,
        phone_number VARCHAR(20),
        is_verified BOOLEAN DEFAULT FALSE,
        last_login TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );";
    client
        .lock()
        .await
        .execute(create_users_table_sql, &[])
        .await?;
    Ok(())
}
