use serde::{Deserialize, Serialize};
use super::NewsId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
  id: NewsId,
  article_id: NewsId,
  reader_id: NewsId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostView {
  article_id: NewsId,
  reader_id: NewsId,
}
