use crate::handlers::products::{
    create_product, delete_product, read_product, read_products, update_product,
};
use actix_web::{web, Scope};

pub fn products_routes() -> Scope {
    web::scope("/users")
        .route("/", web::get().to(read_products))
        .route("/", web::post().to(create_product))
        .route("/{id}", web::get().to(read_product))
        .route("/{id}", web::put().to(update_product))
        .route("/{id}", web::delete().to(delete_product))
}
