use crate::controllers::products::{
    create_product_handler, delete_product_handler, read_product_handler, read_products_handler,
    update_product_handler,
};
use actix_web::web;

pub fn configure_products_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("/", web::post().to(create_product_handler))
            .route("/", web::get().to(read_products_handler))
            .route("/{id}", web::get().to(read_product_handler))
            .route("/{id}", web::put().to(update_product_handler))
            .route("/id}", web::delete().to(delete_product_handler)),
    );
}
