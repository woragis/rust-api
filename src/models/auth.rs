use super::user::UserId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub profile_picture: String,
    pub phone_number: String,
    pub is_verified: bool,
    pub last_login: NaiveDateTime,
}

#[derive(Serialize)]
pub struct AuthUser {
    pub id: UserId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub blog_role: String,
    pub store_role: String,
    pub youtube_role: String,
    pub fanfic_role: String,
    pub profile_picture: Option<String>,
    pub phone_number: Option<String>,
    pub is_verified: bool,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AuthUser {
    pub fn from_row(row: Row) -> Self {
        AuthUser {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            email: row.get("email"),
            role: row.get("role"),
            blog_role: row.get("blog_role"),
            store_role: row.get("store_role"),
            youtube_role: row.get("youtube_role"),
            fanfic_role: row.get("fanfic_role"),
            profile_picture: row.get("profile_picture"),
            phone_number: row.get("phone_number"),
            is_verified: row.get("is_verified"),
            last_login: row.get("last_login"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: AuthUser,
}
