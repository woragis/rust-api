use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::models::store::StoreId;

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: StoreId,               // Primary key
    pub user_id: i32,              // Foreign key to the user
    pub order_date: NaiveDateTime, // Timestamp for when the order was placed
    pub status: String,            // Order status (e.g., pending, completed, canceled)
    pub total_amount: f64,         // Total cost of the order
}

impl Order {
    pub fn from_row(row: Row) -> Self {
        Order {
            id: row.get("id"),
            user_id: row.get("user_id"),
            order_date: row.get("order_date"),
            status: row.get("status"),
            total_amount: row.get("total_amount"),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub status: Option<String>, // Optional status during creation
    pub total_amount: f64,      // Total cost of the order
}
