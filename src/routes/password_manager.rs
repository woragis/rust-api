use actix_web::{
    web::{delete, get, post, put, scope},
    Scope,
};

use crate::handlers::password_manager::{
    delete_data, get_data, get_single_data, insert_data, update_data,
};

pub fn password_manager_routes() -> Scope {
    scope("/password-manager")
        .route("/", get().to(get_data))
        .route("/", post().to(insert_data))
        .route("/{data_id}", get().to(get_single_data))
        .route("/{data_id}", put().to(delete_data))
        .route("/{data_id}", delete().to(update_data))
}
