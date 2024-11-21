use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

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

pub async fn create_product(
    client: Arc<Mutex<Client>>,
    product: &Product,
) -> Result<(), Box<dyn Error>> {
    let create_product_sql = "INSERT INTO products (
    name, description, category, images, price,
    discount, currency, stock, weight, dimensions,
    tags, is_active) VALUES (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);";
    client
        .lock()
        .await
        .execute(
            create_product_sql,
            &[
                &product.name,
                &product.description,
                &product.category,
                &product.images,
                &product.price,
                &product.discount,
                &product.currency,
                &product.stock,
                &product.weight,
                &product.dimensions,
                &product.tags,
                &product.is_active,
            ],
        )
        .await?;

    Ok(())
}

pub async fn read_product(
    client: Arc<Mutex<Client>>,
    id: u32,
) -> Result<Option<Product>, Box<dyn Error>> {
    let read_product_sql = "SELECT * FROM products WHERE id = $1";
    let row = client
        .lock()
        .await
        .query_opt(read_product_sql, &[&id])
        .await?;

    if let Some(row) = row {
        Ok(Some(Product {
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
        }))
    } else {
        Ok(None)
    }
}

pub async fn read_products(client: Arc<Mutex<Client>>) -> Result<Vec<Product>, Box<dyn Error>> {
    let read_products_sql = "SELECT * FROM products;";
    let rows = client.lock().await.query(read_products_sql, &[]).await?;
    let products = rows
        .iter()
        .map(|row| Product {
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
        })
        .collect();

    Ok(products)
}

pub async fn update_product(
    client: Arc<Mutex<Client>>,
    product_id: u32,
    product: &Product,
) -> Result<(), Box<dyn Error>> {
    let update_product_sql = "UPDATE products SET
    name = $1, description = $2, category = $3, images = $4, price = $5,
    discount = $6, currency = $7, stock = $8, weight = $9,
    dimensions = $10, tags = $11, is_active = $12 WHERE id = $13);";
    client
        .lock()
        .await
        .execute(
            update_product_sql,
            &[
                &product.name,
                &product.description,
                &product.category,
                &product.images,
                &product.price,
                &product.discount,
                &product.currency,
                &product.stock,
                &product.weight,
                &product.dimensions,
                &product.tags,
                &product.is_active,
                &product_id,
            ],
        )
        .await?;

    Ok(())
}

pub async fn delete_product(client: Arc<Mutex<Client>>, id: u32) -> Result<(), Box<dyn Error>> {
    let delete_product_sql = "DELETE FROM products WHERE id = $1";
    client
        .lock()
        .await
        .execute(delete_product_sql, &[&id])
        .await?;

    Ok(())
}
