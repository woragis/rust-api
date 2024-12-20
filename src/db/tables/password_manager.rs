use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, Error};

pub const PASSWORD_MANAGER_TABLE: &str = "password_manager";

pub async fn create_password_manager_tables(client: Arc<Mutex<Client>>) -> () {

    match create_password_manager_table(&client).await {
        Ok(_) => info!("Table '{}' created", PASSWORD_MANAGER_TABLE),
        _ => error!("Table '{}' not created", PASSWORD_MANAGER_TABLE),
    }
    ()
}
async fn create_password_manager_table(client: &Arc<Mutex<Client>>) -> Result<(), Error> {
    debug!("Creating news articles table");

    let stmt: String = format!("
        CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL PRIMARY KEY,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    );",
        PASSWORD_MANAGER_TABLE
    );

    client.lock().await.execute(&stmt, &[]).await?;

    Ok(())
}
