use tokio_postgres::{Client, NoTls};
use std::error::Error;

pub struct DbConnection {
    client: Client,
}

impl DbConnection {
    pub async fn new(conn_str: &str) -> Result<Self, Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        Ok(Self {client})
    }
    pub fn get_client(&self) -> &Client {
        &self.client
    }
}
