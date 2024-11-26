use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: i32,            // Primary key
    pub user_id: i32,       // Foreign key to the user
    pub order_date: String, // Timestamp for when the order was placed
    pub status: String,     // Order status (e.g., pending, completed, canceled)
    pub total_amount: f64,  // Total cost of the order
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: i32,           // Foreign key to the user
    pub status: Option<String>, // Optional status during creation
    pub total_amount: f64,      // Total cost of the order
}
