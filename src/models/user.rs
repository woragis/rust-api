use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn find_by_email(
        client: Arc<Mutex<Client>>,
        email: &str,
    ) -> Result<Option<Self>, Error> {
        let find_user_by_email_sql = "SELECT * FROM users WHERE email = $1";
        let row = client
            .lock()
            .await
            .query_opt(find_user_by_email_sql, &[&email])
            .await?;
        Ok(row.map(|row| User {
            id: row.get("id"),
            name: row.get("id"),
            email: row.get("id"),
            password: row.get("id"),
        }))
    }
}
