use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
}
