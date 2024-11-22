use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

use crate::config::db::get_db_string;

pub struct DbConnection {
    client: Arc<Mutex<Client>>,
}

impl DbConnection {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        println!("Connecting to DB....");
        // let db_url = "host=localhost user=postgres password=yourpassword dbname=rust_api";
        let db_url = get_db_string();
        let (client, connection) = tokio_postgres::connect(&db_url, NoTls)
            .await
            .expect("Error connecting to Database");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        println!("Connected to DB!");
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub fn get_client(&self) -> Arc<Mutex<Client>> {
        println!("Receiving DB client");
        self.client.clone()
    }
}
