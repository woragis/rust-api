use super::NewsId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
    id: NewsId,
    article_id: NewsId,
    reader_id: NewsId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostLike {
    article_id: NewsId,
    reader_id: NewsId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutLike {
    article_id: NewsId,
    reader_id: NewsId,
}
