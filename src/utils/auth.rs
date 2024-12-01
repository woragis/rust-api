use super::jwt::verify_jwt;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn verify_ownership(
    client: &web::Data<Arc<Mutex<Client>>>,
    req: &HttpRequest,
    table: &str,
) -> Result<(), impl Responder> {
    match verify_jwt(&req) {
        None => {
            return Err(HttpResponse::Unauthorized().body("You are not the owner of this article"))
        }
        Some(writer_id) => {
            let query = format!("SELECT * FROM {} WHERE id = $1;", table);
            match client.lock().await.query(&query, &[&writer_id]).await {
                Ok(_) => {
                    info!("User ownership verified");
                    Ok(())
                }
                Err(err) => {
                    error!("User cannot update article that is not his: {:?}", err);
                    Err(HttpResponse::Unauthorized().body("You are not the owner of this article"))
                }
            }
        }
    }
}
