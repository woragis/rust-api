use actix_web::{web, Scope};

use crate::handlers::password_manager::{
    delete_data, get_data, get_single_data, insert_data, update_data,
};

pub fn password_manager_routes() -> Scope {
    web::scope("/password-manager")
        .route("/", web::get().to(get_data))
        .route("/", web::post().to(insert_data))
        .route("/{data_id}", web::get().to(get_single_data))
        .route("/{data_id}", web::put().to(delete_data))
        .route("/{data_id}", web::delete().to(update_data))
}
