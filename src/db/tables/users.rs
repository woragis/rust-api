use std::error::Error;
use tokio_postgres::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn create_users_table(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
  let create_users_table_sql = "
  CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  name TEXT,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL
  );";
  client.lock().await.execute(create_users_table_sql, &[])
  .await?;
  Ok(())
}
