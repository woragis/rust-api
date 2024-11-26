use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use super::jwt::verify_jwt;

pub async fn verify_admin(
    client: &web::Data<Arc<Mutex<Client>>>,
    req: &HttpRequest,
) -> Result<bool, HttpResponse> {
    let query = "SELECT role FROM users WHERE id = $1";
    match verify_jwt(&req) {
        Ok(user_id) => match client.lock().await.query_one(query, &[&user_id]).await {
            Ok(row) => {
                let role: String = row.get("role");
                if role == "admin" {
                    Ok(true)
                } else {
                    Err(HttpResponse::Unauthorized().body("Not admin"))
                }
            }
            Err(_) => Err(HttpResponse::InternalServerError().body("Erro ao procurar usuario")),
        },
        Err(_) => Err(HttpResponse::InternalServerError().body("Erro no token")),
    }
}
