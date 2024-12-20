use crate::models::user::UserId;
use crate::{db::tables::users::USERS_TABLE, models::user::User};
use crate::utils::emails::send_email;
use crate::utils::jwt::verify_jwt;
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn recover_password(
    client: Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Reading user profile");
    let user_id: UserId = verify_jwt(&req).expect("oi");
    let stmt: String = format!("SELECT * FROM {} WHERE id = $1;", USERS_TABLE);
    match client.lock().await.query_opt(&stmt, &[&user_id]).await {
        Ok(Some(row)) => {
            let user: User = User::from_row(row);
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

pub async fn verify_email(client: Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    debug!("Sending email verification");
    let user_id: UserId = verify_jwt(&req).expect("oi");
    let stmt: String= format!("SELECT email FROM {} WHERE id = $1;", USERS_TABLE);
    match client.lock().await.query_one(&stmt, &[&user_id]).await {
        Ok(row) => {
            let email: String = row.get("email");
            let subject: &str = "Verify your email";
            let body: &str = "hi bitch";
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
                    HttpResponse::InternalServerError().body("Error sending verification email")
                }
            }
        }
        Err(err) => {
            error!("Error finding user with id={}: {:?}", user_id, err);
            HttpResponse::InternalServerError().body("Error finding user email")
        }
    }
}
