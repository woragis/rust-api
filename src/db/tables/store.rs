use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, Error};

use crate::db::tables::users::USERS_TABLE;

pub const PRODUCTS_TABLE: &str = "store_products";
pub const ORDERS_TABLE: &str = "store_orders";

pub async fn create_store_tables(client: Arc<Mutex<Client>>) -> () {
    match create_products_table(&client).await {
        Ok(_) => info!("Table '{}' created", PRODUCTS_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", PRODUCTS_TABLE, err),
    }
    match create_orders_table(&client).await {
        Ok(_) => info!("Table '{}' created", ORDERS_TABLE),
        Err(err) => error!("Table '{}' not created: {:?}", ORDERS_TABLE, err),
    }
}

async fn create_products_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating products table");

    let stmt: String = format!(
        "
        CREATE TABLE IF NOT EXISTS {} (
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
    );",
        PRODUCTS_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}

pub async fn create_orders_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating orders table");

    let stmt: String = format!(
        "
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        user_id BIGINT NOT NULL REFERENCES {}(id) ON DELETE CASCADE,
        order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        status VARCHAR(20) NOT NULL DEFAULT 'pending',
        total_amount NUMERIC(10, 2) NOT NULL
    );",
        ORDERS_TABLE, USERS_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}
