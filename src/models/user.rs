use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "admin")]
    Admin,
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Role::User),
            "admin" => Ok(Role::Admin),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::User => "user".to_string(),
            Role::Admin => "admin".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub profile_picture: Option<String>,
    pub phone_number: Option<String>,
    pub is_verified: bool,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn from_row(row: Row) -> Self {
        User {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            email: row.get("email"),
            password: row.get("password"),
            role: row.get("role"),
            profile_picture: row.get("profile_picture"),
            phone_number: row.get("phone_number"),
            is_verified: row.get("is_verified"),
            last_login: row.get("last_login"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub profile_picture: String,
    pub phone_number: String,
    pub is_verified: bool,
    pub last_login: NaiveDateTime,
}
