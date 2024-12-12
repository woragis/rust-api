use actix_web::{HttpResponse, Responder};
use log::info;

use crate::{
    config::encryption::BLOCK_SIZE,
    utils::encryption::{generate_key, vec_to_string},
};

pub async fn generate_new_key() -> impl Responder {
    let key = vec_to_string(&generate_key(BLOCK_SIZE));
    info!("Succeffully deleted data from password manager app");
    HttpResponse::Ok().json(key)
}
