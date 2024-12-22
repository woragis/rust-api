use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{models::user::UserId, shared::types::Id};

pub type BlogId = Id;

#[derive(Deserialize, Serialize)]
pub struct BlogSubscription {
    id: BlogId,
    user_id: UserId,
    blogger_id: BlogId,
    subscribed_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct SubscribeRequest {
    pub blogger_id: BlogId,
}
