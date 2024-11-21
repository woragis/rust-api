use actix_web::{web, HttpResponse, Responder};

use crate::db;
use db::connection::DbConnection;
use db::operations::products::Product;
use db::operations::products::{
    create_product, delete_product, read_product, read_products, update_product,
};

pub async fn create_product_handler(
    db: web::Data<DbConnection>,
    user: web::Json<Product>,
) -> impl Responder {
    let client = db.get_client();
    if let Err(err) = create_product(client, &user.into_inner()).await {
        return HttpResponse::InternalServerError().body(format!("Error: {}", err));
    }
    HttpResponse::Created().finish()
}

pub async fn read_product_handler(
    db: web::Data<DbConnection>,
    product_id: web::Path<u32>,
) -> impl Responder {
    let client = db.get_client();
    match read_product(client, product_id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn read_products_handler(db: web::Data<DbConnection>) -> impl Responder {
    let client = db.get_client();
    match read_products(client).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn update_product_handler(
    db: web::Data<DbConnection>,
    product_id: web::Path<u32>,
    product: web::Json<Product>,
) -> impl Responder {
    let client = db.get_client();
    if let Err(err) = update_product(client, product_id.into_inner(), &product.into_inner()).await {
        return HttpResponse::InternalServerError().body(format!("Error: {}", err));
    }
    HttpResponse::Ok().finish()
}

pub async fn delete_product_handler(
    db: web::Data<DbConnection>,
    product_id: web::Path<u32>,
) -> impl Responder {
    let client = db.get_client();
    if let Err(err) = delete_product(client, product_id.into_inner()).await {
        return HttpResponse::InternalServerError().body(format!("Error: {}", err));
    }
    HttpResponse::Ok().finish()
}
