use crate::shared::types::Id;
use bytes::BytesMut;
use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;
use tokio_postgres::types::{IsNull, Type};
use tokio_postgres::Row;

pub type UserId = Id;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl<'a> FromSql<'a> for Role {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        if ty.name() != "role_enum" {
            return Err(format!("Unexpected type: {}", ty.name()).into());
        }
        let value = std::str::from_utf8(raw)?;
        match value {
            "user" => Ok(Role::User),
            "admin" => Ok(Role::Admin),
            _ => Err(format!("Invalid enum value: {}", value).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "role_enum"
    }
}

impl ToSql for Role {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        if ty.name() != "role_enum" {
            return Err(format!("Unexpected type: {}", ty.name()).into());
        }

        let value = match self {
            Role::User => "user",
            Role::Admin => "admin",
        };
        out.extend_from_slice(value.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "role_enum"
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.to_sql(ty, out)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: UserId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub decrypted_password: String,
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

impl User {
    pub fn from_row(row: Row) -> Self {
        User {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            email: row.get("email"),
            password: row.get("password"),
            decrypted_password: row.get("decrypted_password"),
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
    pub id: UserId,
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
    pub decrypted_password: String,
    pub role: String,
    pub blog_role: String,
    pub store_role: String,
    pub youtube_role: String,
    pub fanfic_role: String,
    pub profile_picture: String,
    pub phone_number: String,
    pub is_verified: bool,
    pub last_login: NaiveDateTime,
}

/*
#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    Draft,
    Published,
    Archived,
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(Status::Draft),
            "published" => Ok(Status::Published),
            "archived" => Ok(Status::Archived),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

impl FromSql for Status {
    fn from_sql(ty: &tokio_postgres::types::Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let raw_str = std::str::from_utf8(raw)?.to_owned();
        Status::from_str(&raw_str).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

impl ToSql for Status {
    fn to_sql(&self, ty: &tokio_postgres::types::Type, out: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let value = match *self {
            Status::Draft => "draft",
            Status::Published => "published",
            Status::Archived => "archived",
        };
        out.extend_from_slice(value.as_bytes());
        Ok(())
    }
}
*/
