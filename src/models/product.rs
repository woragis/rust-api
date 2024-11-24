use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
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
    pub created_at: String,
    pub updated_at: String,
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
    pub created_at: String,
    pub updated_at: String,
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
    pub created_at: String,
    pub updated_at: String,
}
