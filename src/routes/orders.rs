use crate::{
    config::routes::ORDERS_ROUTES,
    handlers::orders::{create_order, delete_order, read_order, read_orders},
};
use actix_web::{web, Scope};

pub fn orders_routes() -> Scope {
    web::scope(ORDERS_ROUTES)
        .route("/", web::get().to(read_orders))
        .route("/", web::post().to(create_order))
        .route("/{id}", web::get().to(read_order))
        .route("/{id}", web::delete().to(delete_order))
}
