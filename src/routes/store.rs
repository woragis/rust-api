use crate::handlers::store::orders::{create_order, delete_order, read_order, read_orders};
use crate::handlers::store::products::{
    create_product, delete_product, read_product, read_products, update_product,
};
use actix_web::{
    web::{delete, get, post, put, scope},
    Scope,
};

pub fn products_routes() -> Scope {
    scope("/store/products")
        .route("/", get().to(read_products))
        .route("/", post().to(create_product))
        .route("/{product_id}", get().to(read_product))
        .route("/{product_id}", put().to(update_product))
        .route("/{product_id}", delete().to(delete_product))
}

pub fn orders_routes() -> Scope {
    scope("/store/orders")
        .route("/", get().to(read_orders))
        .route("/", post().to(create_order))
        .route("/{order_id}", get().to(read_order))
        .route("/{order_id}", delete().to(delete_order))
}
