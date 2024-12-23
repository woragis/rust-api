use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::shared::types::Id;

pub type BlogId = Id;

pub enum BlogPostVisibility {
    Hidden,
    Visible,
    Private,
}

#[derive(Deserialize, Serialize)]
pub struct BlogPost {
    pub id: BlogId,
    pub title: String,
    pub body: String,
    pub author_id: BlogId,
    pub visibility: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl BlogPost {
    pub fn from_row(row: Row) -> Self {
        BlogPost {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            author_id: row.get("author_id"),
            visibility: row.get("visibility"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateBlogPost {
    pub title: String,
    pub body: String,
    pub visibility: String,
}

#[derive(Deserialize)]
pub struct UpdateBlogPost {
    pub title: String,
    pub body: String,
    pub visibility: String,
}
