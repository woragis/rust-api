use tokio_postgres::{Client, NoTls};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DbConnection {
    client: Arc<Mutex<Client>>,
}

impl DbConnection {
    pub async fn new(conn_str: &str) -> Result<Self, Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }
    pub fn get_client(&self) -> Arc<Mutex<Client>> {
        self.client.clone()
    }
}
