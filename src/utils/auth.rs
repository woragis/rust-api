use super::jwt::verify_jwt;
use actix_web::{web, HttpRequest, HttpResponse};
use log::{error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn verify_ownership(
    client: &web::Data<Arc<Mutex<Client>>>,
    req: &HttpRequest,
    table: &str,
    id: &str,
) -> Result<bool, bool> {
    match verify_jwt(&req) {
        None => {
            HttpResponse::Unauthorized().body("You are not the owner of this article");
            Err(false)
        }
        Some(writer_id) => {
            let query = format!("SELECT * FROM {} WHERE {} = $1;", table, id);
            match client.lock().await.query(&query, &[&writer_id]).await {
                Ok(_) => {
                    info!("User ownership verified");
                    Ok(true)
                }
                Err(err) => {
                    error!("User cannot update article that is not his: {:?}", err);
                    HttpResponse::Unauthorized().body("You are not the owner of this article");
                    Err(false)
                }
            }
        }
    }
}
