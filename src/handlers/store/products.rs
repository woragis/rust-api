use crate::db::tables::store::PRODUCTS_TABLE;
use crate::models::store::product::{
    CreateProductRequest, Product, ProductId, UpdateProductRequest,
};
use crate::utils::admin::verify_admin;
use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_product(
    client: Data<Arc<Mutex<Client>>>,
    product: Json<CreateProductRequest>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for creating a product");
    match verify_admin(&client, &req).await {
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => warn!("Admin verification failed"),
        _ => error!("Error verifying admin"),
    };

    debug!("Inserting new product into the database");
    let query = format!("INSERT INTO {} (
        name, description, category, images, price,
        discount, currency, stock, weight, dimensions,
        tags, is_active) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id", PRODUCTS_TABLE);
    match client
        .lock()
        .await
        .query_one(
            &query,
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
            let id: ProductId = row.get("id");
            info!("Successfully created product with id={}", id);
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
            error!("Failed to create product: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to create product")
        }
    }
}

pub async fn read_product(
    client: Data<Arc<Mutex<Client>>>,
    product_id: Path<ProductId>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for reading a product");
    match verify_admin(&client, &req).await {
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => warn!("Admin verification failed"),
        _ => error!("Error verifying admin"),
    };

    debug!("Querying product with id={}", product_id);
    let query = format!("SELECT * FROM {} WHERE id = $1", PRODUCTS_TABLE);
    match client.lock().await.query_opt(&query, &[&*product_id]).await {
        Ok(Some(row)) => {
            let product = Product::from_row(row);
            info!("Successfully retrieved product with id={}", product.id);
            HttpResponse::Ok().json(product)
        }
        Ok(None) => {
            warn!("No product found with id={}", product_id);
            HttpResponse::NotFound().body(format!("Product '{}' not found", product_id))
        }
        Err(err) => {
            error!(
                "Failed to retrieve product with id={}: {:?}",
                product_id, err
            );
            HttpResponse::NotFound().body("Product not found")
        }
    }
}

pub async fn read_products(client: Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    debug!("Verifying admin privileges for reading all products");
    match verify_admin(&client, &req).await {
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => warn!("Admin verification failed"),
        _ => error!("Error verifying admin"),
    };

    debug!("Querying all products from the database");
    let query = format!("SELECT * FROM {}", PRODUCTS_TABLE);
    match client.lock().await.query(&query, &[]).await {
        Ok(rows) => {
            let products: Vec<Product> =
                rows.into_iter().map(|row| Product::from_row(row)).collect();
            info!("Successfully retrieved all products");
            HttpResponse::Ok().json(products)
        }
        Err(err) => {
            error!("Failed to retrieve products: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to fetch products")
        }
    }
}

pub async fn update_product(
    client: Data<Arc<Mutex<Client>>>,
    product_id: Path<ProductId>,
    product: Json<UpdateProductRequest>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for updating a product");
    match verify_admin(&client, &req).await {
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => warn!("Admin verification failed"),
        _ => error!("Error verifying admin"),
    };

    debug!("Updating product with id={}", product_id);
    let query = format!("UPDATE {} SET
        name = $1, description = $2, category = $3, images = $4, price = $5,
        discount = $6, currency = $7, stock = $8, weight = $9,
        dimensions = $10, tags = $11, is_active = $12 WHERE id = $13);", PRODUCTS_TABLE);
    match client
        .lock()
        .await
        .execute(
            &query,
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
            info!("Successfully updated product with id={}", product_id);
            HttpResponse::Ok().body("Product updated")
        }
        Ok(_) => {
            warn!("No product found with id={}", product_id);
            HttpResponse::NotFound().body(format!("Product '{}' not found", product_id))
        }
        Err(err) => {
            error!("Failed to update product with id={}: {:?}", product_id, err);
            HttpResponse::InternalServerError().body("Failed to update product")
        }
    }
}

pub async fn delete_product(
    client: Data<Arc<Mutex<Client>>>,
    product_id: Path<ProductId>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for deleting a product");
    match verify_admin(&client, &req).await {
        Ok(true) => info!("Admin privileges verified"),
        Ok(false) => warn!("Admin verification failed"),
        _ => error!("Error verifying admin"),
    };

    debug!("Deleting Product with id={}", product_id);
    let query = format!("DELETE FROM {} WHERE id = $1", PRODUCTS_TABLE);
    match client.lock().await.execute(&query, &[&*product_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            info!("Successfully deleted product with id={}", product_id);
            HttpResponse::Ok().body("Product deleted")
        }
        Ok(_) => {
            warn!("No product found with id={}", product_id);
            HttpResponse::NotFound().body(format!("Product '{}' not found", product_id))
        }
        Err(err) => {
            error!("Failed to delete product with id={}: {:?}", product_id, err);
            HttpResponse::InternalServerError().body("Failed to delete product")
        }
    }
}
