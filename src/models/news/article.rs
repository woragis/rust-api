use super::NewsId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio_postgres::Row;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    Published,
    Draft,
    Archived,
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "published" => Ok(Status::Published),
            "draft" => Ok(Status::Draft),
            "archived" => Ok(Status::Archived),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Published => "published".to_string(),
            Status::Draft => "draft".to_string(),
            Status::Archived => "archived".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsArticle {
    pub id: NewsId,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub writer_id: Option<NewsId>,
    // pub category_id: Option<NewsId>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewsArticle {
    pub fn from_row(row: Row) -> Self {
        NewsArticle {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            summary: row.get("summary"),
            writer_id: row.get("writer_id"),
            // category_id: row.get("category_id"),
            status: row.get("status"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewsArticleRequest {
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    // pub category_id: Option<NewsId>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateNewsArticleRequest {
    id: NewsId,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    // pub category_id: Option<NewsId>,
    pub status: String,
    pub published_at: Option<NaiveDateTime>,
}
