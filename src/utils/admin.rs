use super::jwt::verify_jwt;
use actix_web::{web, HttpRequest, HttpResponse};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn verify_admin(
    client: &web::Data<Arc<Mutex<Client>>>,
    req: &HttpRequest,
) -> Result<bool, HttpResponse> {
    let query = "SELECT role FROM users WHERE id = $1";
    let user_id = verify_jwt(&req)?;
    // .ok_or_else(|| {
    //     error!("Failed to verify jwt",);
    //     HttpResponse::InternalServerError().body("Error while fetching user role")
    // })?;

    let row = client
        .lock()
        .await
        .query_one(query, &[&user_id])
        .await
        .map_err(|err| {
            error!(
                "Database query failed while checking role for user_id={}: {:?}",
                user_id, err
            );
            HttpResponse::InternalServerError().body("Error while fetching user role")
        })?;

    let role: String = row.get("role");
    debug!("Retrieved role for user_id={}: {}", user_id, role);

    if role != "admin" {
        warn!("User with user_id={} is not an admin", user_id);
        Ok(false)
    } else {
        info!("User with user_id={} is an admin", user_id);
        Ok(true)
    }
}
