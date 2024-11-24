use crate::models::product::{CreateProductRequest, Product, UpdateProductRequest};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_product(
    client: web::Data<Arc<Mutex<Client>>>,
    product: web::Json<CreateProductRequest>,
) -> impl Responder {
    println!("Creating Product");
    let query = "INSERT INTO products (
        name, description, category, images, price,
        discount, currency, stock, weight, dimensions,
        tags, is_active) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id";
    match client
        .lock()
        .await
        .query_one(
            query,
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
        .await
    {
        Ok(row) => {
            let id = row.get("id");
            println!("Created Product: '{}'", id);
            HttpResponse::Created().json(Product {
                id,
                name: product.name.clone(),
                description: product.description.clone(),
                category: product.category.clone(),
                images: product.images.clone(),
                price: product.price.clone(),
                discount: product.discount.clone(),
                currency: product.currency.clone(),
                stock: product.stock.clone(),
                weight: product.weight.clone(),
                dimensions: product.dimensions.clone(),
                tags: product.tags.clone(),
                is_active: product.is_active.clone(),
                created_at: product.created_at.clone(),
                updated_at: product.updated_at.clone(),
            })
        }
        Err(err) => {
            eprintln!("Failed to create product: {}", err);
            HttpResponse::InternalServerError().body("Failed to create product")
        }
    }
}

pub async fn read_product(
    client: web::Data<Arc<Mutex<Client>>>,
    product_id: web::Path<u32>,
) -> impl Responder {
    println!("Reading Product '{}'", product_id);
    let query = "SELECT * FROM products WHERE id = $1";
    match client.lock().await.query_one(query, &[&*product_id]).await {
        Ok(row) => {
            let user = Product {
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
            };
            println!("Read Product '{}'", user.id);
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            eprintln!("Product not found: {}", err);
            HttpResponse::NotFound().body("Product not found")
        }
    }
}

pub async fn read_products(client: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    println!("Reading Products");
    let query = "SELECT * FROM products";
    match client.lock().await.query(query, &[]).await {
        Ok(rows) => {
            let products: Vec<Product> = rows
                .into_iter()
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
            println!("Read Products");
            HttpResponse::Ok().json(products)
        }
        Err(err) => {
            eprintln!("Error fetching products: {}", err);
            HttpResponse::InternalServerError().body("Failed to fetch products")
        }
    }
}

pub async fn update_product(
    client: web::Data<Arc<Mutex<Client>>>,
    product_id: web::Path<u32>,
    product: web::Json<UpdateProductRequest>,
) -> impl Responder {
    println!("Updating product '{}'", product_id);
    let query = "UPDATE products SET
        name = $1, description = $2, category = $3, images = $4, price = $5,
        discount = $6, currency = $7, stock = $8, weight = $9,
        dimensions = $10, tags = $11, is_active = $12 WHERE id = $13);";
    match client
        .lock()
        .await
        .execute(
            query,
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
                &*product_id,
            ],
        )
        .await
    {
        Ok(rows_updated) if rows_updated > 0 => {
            println!("Updated product '{}'", product_id);
            HttpResponse::Ok().body("Product updated")
        }
        Ok(_) => HttpResponse::NotFound().body(format!("Product '{}' not found", product_id)),
        Err(err) => {
            eprintln!("Failed to update product: {}", err);
            HttpResponse::InternalServerError().body("Failed to update product")
        }
    }
}

pub async fn delete_product(
    client: web::Data<Arc<Mutex<Client>>>,
    product_id: web::Path<u32>,
) -> impl Responder {
    println!("Deleting Product '{}'", product_id);
    let query = "DELETE FROM products WHERE id = $1";
    match client.lock().await.execute(query, &[&*product_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            println!("Deleted Product '{}'", product_id);
            HttpResponse::Ok().body("Product deleted")
        }
        Ok(_) => HttpResponse::NotFound().body(format!("Product '{}' not found", product_id)),
        Err(err) => {
            eprintln!("Failed to delete product: {}", err);
            HttpResponse::InternalServerError().body("Failed to delete product")
        }
    }
}
