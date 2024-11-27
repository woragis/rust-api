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
    debug!("Starting admin verification");

    let query = "SELECT role FROM users WHERE id = $1";
    match verify_jwt(&req) {
        Ok(user_id) => {
            debug!("JWT verification successfully, user_id={}", user_id);

            match client.lock().await.query_one(query, &[&user_id]).await {
                Ok(row) => {
                    let role: String = row.get("role");
                    debug!("Retrieved role for user_id={}: {}", user_id, role);

                    if role == "admin" {
                        info!("User with user_id={} is an admin", user_id);
                        Ok(true)
                    } else {
                        warn!("User with user_id={} is not an admin", user_id);
                        Err(HttpResponse::Unauthorized().body("Not admin"))
                    }
                }
                Err(err) => {
                    error!(
                        "Database query failed while checking role for user_id={}: {:?}",
                        user_id, err
                    );
                    Err(HttpResponse::InternalServerError().body("Error while fetching user role"))
                }
            }
        }
        Err(err) => {
            error!("JWT verification failed: {:?}", err);
            Err(HttpResponse::InternalServerError().body("Invalid token"))
        }
    }
}
