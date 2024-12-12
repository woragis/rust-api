use super::NewsId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: NewsId,
    pub article_id: NewsId,
    pub reader_id: NewsId,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Comment {
    pub fn from_row(row: Row) -> Self {
        Comment {
            id: row.get("id"),
            article_id: row.get("article_id"),
            reader_id: row.get("reader_id"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateComment {
    pub content: String,
}

#[derive(Deserialize)]
pub struct EditComment {
    pub id: NewsId,
    pub article_id: NewsId,
    pub content: String,
}

#[derive(Deserialize)]
pub struct DeleteComment {
    pub id: NewsId,
}