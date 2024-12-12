use log::{debug, error, info};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub const PRODUCTS_TABLE: &str = "store_products";
pub const ORDERS_TABLE: &str = "store_orders";

pub async fn create_store_tables(client: Arc<Mutex<Client>>) -> () {
    match create_products_table(&client).await {
        Ok(_) => info!("Table 'store_products' created"),
        Err(_) => error!("Table 'store_products' not created"),
    }
    match create_orders_table(&client).await {
        Ok(_) => info!("Table 'store_orders' created"),
        Err(_) => error!("Table 'store_orders' not created"),
    }
}

async fn create_products_table(client: &Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating products table");

    let create_users_table_sql = "
        CREATE TABLE IF NOT EXISTS store_products (
        id BIGSERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        description TEXT,
        category VARCHAR(100),
        images TEXT[],
        price DECIMAL(10, 2) NOT NULL,
        discount DECIMAL(5, 2) DEFAULT 0.00,
        currency VARCHAR(3) DEFAULT 'USD',
        stock INT DEFAULT 0,
        weight DECIMAL(10, 2),
        dimensions JSONB,
        tags TEXT[],
        is_active BOOLEAN DEFAULT TRUE,
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

pub async fn create_orders_table(client: &Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    debug!("Creating orders table");

    let create_users_table_sql = "
        CREATE TABLE IF NOT EXISTS store_orders (
        id BIGSERIAL PRIMARY KEY,
        user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
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
