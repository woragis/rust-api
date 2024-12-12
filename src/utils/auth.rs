use super::jwt::verify_jwt;
use actix_web::{web::Data, HttpRequest, HttpResponse};
use log::{error, info};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn verify_ownership(
    client: &Data<Arc<Mutex<Client>>>,
    req: &HttpRequest,
    table: &str,
    id: &str,
) -> Result<bool, HttpResponse> {
    let user_id = verify_jwt(&req)?;

    let query = format!("SELECT * FROM {} WHERE {} = $1;", table, id);

    if let Err(err) = client.lock().await.query(&query, &[&user_id]).await {
        error!("Failed to verify jwt: {:?}", err);
        Err(HttpResponse::InternalServerError().body("Error verifying user_id"))
    } else {
        info!("User ownership verified");
        Ok(true)
    }
}
