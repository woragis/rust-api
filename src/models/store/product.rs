use crate::models::store::StoreId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: StoreId,
    pub name: String,
    pub description: String,
    pub category: String,
    pub images: Vec<String>,
    pub price: f32,
    pub discount: f32,
    pub currency: String,
    pub stock: u32,
    pub weight: u32,
    pub dimensions: String,
    pub tags: Vec<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Product {
    pub fn from_row(row: Row) -> Self {
        Product {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            category: row.get("category"),
            images: row.get("images"),
            price: row.get("price"),
            discount: row.get("discount"),
            currency: row.get("currency"),
            stock: row.get("stock"),
            weight: row.get("weight"),
            dimensions: row.get("dimensions"),
            tags: row.get("tags"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub images: Vec<String>,
    pub price: f32,
    pub discount: f32,
    pub currency: String,
    pub stock: u32,
    pub weight: u32,
    pub dimensions: String,
    pub tags: Vec<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UpdateProductRequest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub images: Vec<String>,
    pub price: f32,
    pub discount: f32,
    pub currency: String,
    pub stock: u32,
    pub weight: u32,
    pub dimensions: String,
    pub tags: Vec<String>,
    pub is_active: bool,
}
