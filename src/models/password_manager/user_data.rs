use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::shared::types::Id;

#[derive(Deserialize, Serialize)]
pub struct ServiceData {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl ServiceData {
    pub fn from_row(row: Row) -> Self {
        ServiceData {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            username: row.get("username"),
            password: row.get("password"),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateData {
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateData {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}
