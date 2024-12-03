use crate::models::user::User;
use crate::utils::emails::send_email;
use crate::utils::jwt::verify_jwt;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn recover_password(
    client: web::Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Reading user profile");
    match verify_jwt(&req) {
        Some(user_id) => {
            let query = "SELECT * FROM users WHERE id = $1;";
            match client.lock().await.query_opt(query, &[&user_id]).await {
                Ok(Some(row)) => {
                    let user = User::from_row(row);
                    info!("Successfully retrieved user profile with id={}", user_id);
                    HttpResponse::Ok().json(user)
                }
                Ok(None) => {
                    warn!("No user profile found with id={}", user_id);
                    HttpResponse::NotFound().body(format!("User '{}' not found", user_id))
                }
                Err(err) => {
                    error!(
                        "Failed to retrieve user profile with id={}: {:?}",
                        user_id, err
                    );
                    HttpResponse::InternalServerError().body("Failed to read profile")
                }
            }
        }
        None => {
            error!("Failed to verify JWT");
            HttpResponse::InternalServerError().body("Failed to verify token")
        }
    }
}

pub async fn verify_email(
    client: web::Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Sending email verification");
    match verify_jwt(&req) {
        Some(user_id) => {
            let query = "
            SELECT email FROM users WHERE id = $1;";
            match client.lock().await.query_one(query, &[&user_id]).await {
                Ok(row) => {
                    let email: String = row.get("email");
                    let subject = "Verify your email";
                    let body = "hi bitch";
                    match send_email(&email, subject, body).await {
                        Ok(_) => {
                            info!(
                                "Successfully sent email verification to user with email='{}'",
                                email
                            );
                            HttpResponse::Ok().body("Email verification sent")
                        }
                        Err(_) => {
                            error!("Error sending verification email");
                            HttpResponse::InternalServerError()
                                .body("Error sending verification email")
                        }
                    }
                }
                Err(err) => {
                    error!("Error finding user with id={}: {:?}", user_id, err);
                    HttpResponse::InternalServerError().body("Error finding user email")
                }
            }
        }
        None => {
            error!("Failed to send verifification email");
            HttpResponse::InternalServerError().body("Failed to send verification email")
        }
    }
}
