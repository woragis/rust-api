use log::{debug, error, info};
use std::error::Error;
use std::sync::Arc;
use tokio::{spawn, sync::Mutex};
use tokio_postgres::{connect, Client, NoTls};

use crate::config::db::get_db_string;

pub struct DbConnection {
    client: Arc<Mutex<Client>>,
}

impl DbConnection {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        debug!("Estabilishing database connection");
        let db_url = get_db_string();
        let (client, connection) = connect(&db_url, NoTls)
            .await
            .expect("Error connecting to Database");

        spawn(async move {
            if let Err(err) = connection.await {
                error!("Failed to connect to database: {:?}", err);
            }
        });

        info!("Successfully connected to db!");
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub fn get_client(&self) -> Arc<Mutex<Client>> {
        info!("Successfully received db client");
        self.client.clone()
    }
}
