use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::{
    shared::types::Id,
    utils::encryption::{decrypt, encrypt, vec_to_string},
};

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
    pub fn decrypt_row(row: Row, key: &[u8]) -> Self {
        ServiceData {
            id: row.get("id"),
            name: vec_to_string(&decrypt(key, row.get("name"))),
            email: vec_to_string(&decrypt(key, row.get("email"))),
            username: vec_to_string(&decrypt(key, row.get("username"))),
            password: vec_to_string(&decrypt(key, row.get("password"))),
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

impl CreateData {
    pub fn encrypt_data(key: &[u8], data: Json<CreateData>, block_size: usize) -> Self {
        CreateData {
            name: vec_to_string(&encrypt(key, data.name.as_bytes(), block_size)),
            email: vec_to_string(&encrypt(key, data.email.as_bytes(), block_size)),
            username: vec_to_string(&encrypt(key, data.username.as_bytes(), block_size)),
            password: vec_to_string(&encrypt(key, data.password.as_bytes(), block_size)),
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateData {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl UpdateData {
    pub fn encrypt_data(key: &[u8], data: Json<UpdateData>, block_size: usize) -> Self {
        UpdateData {
            id: data.id,
            name: vec_to_string(&encrypt(key, data.name.as_bytes(), block_size)),
            email: vec_to_string(&encrypt(key, data.email.as_bytes(), block_size)),
            username: vec_to_string(&encrypt(key, data.username.as_bytes(), block_size)),
            password: vec_to_string(&encrypt(key, data.password.as_bytes(), block_size)),
        }
    }
}
