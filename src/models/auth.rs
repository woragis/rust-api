use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u32,
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
    pub token: String,
}
