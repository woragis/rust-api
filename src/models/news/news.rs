use super::NewsId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CategoryType {
    Technology,
    Health,
    Business,
    Education,
    // Add other categories here
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsCategories {
    id: NewsId,
    name: String,
    description: String,
    created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsComments {
    id: NewsId,
    comment_id: NewsId,
    article_id: NewsId,
    reader_id: NewsId,
    content: String,
    created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsTags {
    id: NewsId,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsArticleTags {
    id: NewsId,
    article_id: NewsId,
    tag_id: NewsId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsLikes {
    id: NewsId,
    article_id: NewsId,
    reader_id: NewsId,
    created_at: NewsId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewsArticleViews {
    id: NewsId,
    article_id: NewsId,
    reader_id: NewsId,
    ip_address: String, // INET type
    view_timestamp: NaiveDateTime,
}
