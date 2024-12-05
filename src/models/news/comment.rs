use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::NewsId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
  pub id: NewsId,
  pub article_id: NewsId,
  pub reader_id: NewsId,
  pub content: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostComment {
  pub article_id: NewsId,
  pub reader_id: NewsId,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutComment {
  pub article_id: NewsId,
  pub reader_id: NewsId,
  pub content: String,
}
