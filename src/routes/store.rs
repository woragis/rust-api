use crate::handlers::store::orders::{create_order, delete_order, read_order, read_orders};
use crate::handlers::store::products::{
    create_product, delete_product, read_product, read_products, update_product,
};
use actix_web::{web, Scope};

pub fn products_routes() -> Scope {
    web::scope("/store/products")
        .route("/", web::get().to(read_products))
        .route("/", web::post().to(create_product))
        .route("/{product_id}", web::get().to(read_product))
        .route("/{product_id}", web::put().to(update_product))
        .route("/{product_id}", web::delete().to(delete_product))
}

pub fn orders_routes() -> Scope {
    web::scope("/store/orders")
        .route("/", web::get().to(read_orders))
        .route("/", web::post().to(create_order))
        .route("/{order_id}", web::get().to(read_order))
        .route("/{order_id}", web::delete().to(delete_order))
}
