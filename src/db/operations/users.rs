use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub async fn create_user(
    client: Arc<Mutex<Client>>,
    name: &str,
    email: &str,
) -> Result<(), Box<dyn Error>> {
    let create_user_sql = "INSERT INTO users (name, email) VALUES ($1, $2);";
    client
        .lock()
        .await
        .execute(create_user_sql, &[&name, &email])
        .await?;

    Ok(())
}

pub async fn read_user(
    client: Arc<Mutex<Client>>,
    id: u32,
) -> Result<Option<User>, Box<dyn Error>> {
    let read_user_sql = "SELECT * FROM users WHERE id = $1";
    let row = client.lock().await.query_opt(read_user_sql, &[&id]).await?;

    if let Some(row) = row {
        Ok(Some(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        }))
    } else {
        Ok(None)
    }
}

pub async fn read_users(client: Arc<Mutex<Client>>) -> Result<Vec<User>, Box<dyn Error>> {
    let read_users_sql = "SELECT * FROM users;";
    let rows = client.lock().await.query(read_users_sql, &[]).await?;
    let users = rows
        .iter()
        .map(|row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        })
        .collect();

    Ok(users)
}

pub async fn update_user(
    client: Arc<Mutex<Client>>,
    id: u32,
    name: &str,
    email: &str,
) -> Result<(), Box<dyn Error>> {
    let update_user_sql = "UPDATE users SET name = $1, email = $2 WHERE id = $3";
    client
        .lock()
        .await
        .execute(update_user_sql, &[&name, &email, &id])
        .await?;

    Ok(())
}

pub async fn delete_user(client: Arc<Mutex<Client>>, id: u32) -> Result<(), Box<dyn Error>> {
    let delete_user_sql = "DELETE FROM users WHERE id = $1";
    client.lock().await.execute(delete_user_sql, &[&id]).await?;

    Ok(())
}
