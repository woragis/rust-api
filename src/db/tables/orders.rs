use log::debug;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_orders_table(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating orders table");
    let create_users_table_sql = "
        CREATE TABLE IF NOT EXISTS orders (
        id BIGSERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        status VARCHAR(20) NOT NULL DEFAULT 'pending',
        total_amount NUMERIC(10, 2) NOT NULL
    );";
    client
        .lock()
        .await
        .execute(create_users_table_sql, &[])
        .await?;
    Ok(())
}
