use crate::models::auth::UpdateProfileRequest;
use crate::models::user::User;
use crate::utils::bcrypt::hash_password;
use crate::utils::jwt::verify_jwt;
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn read_profile(client: Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    debug!("Reading user profile");
    let user_id = verify_jwt(&req).expect("oi");
    let query: &str = "SELECT * FROM users WHERE id = $1;";
    match client.lock().await.query_opt(query, &[&user_id]).await {
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

pub async fn update_profile(
    client: Data<Arc<Mutex<Client>>>,
    form: Json<UpdateProfileRequest>,
    req: HttpRequest,
) -> impl Responder {
    debug!("Updating user profile");
    let hashed_password = hash_password(&form.password);
    let user_id = verify_jwt(&req).expect("oi");
    let query: &str = "
            UPDATE users SET
            first_name = $1, last_name = $2, email = $3,
            password = $4, decrypted_password = $5, role = $6,
            blog_role = $7, store_role = $8, youtube_role = $9, fanfic_role = $10,
            profile_picture = $11, phone_number = $12,
            is_verified = $13, last_login = $14, updated_at = CURRENT_TIMESTAMP
            WHERE id = $15";
    match client
        .lock()
        .await
        .execute(
            query,
            &[
                &form.first_name,
                &form.last_name,
                &form.email,
                &form.password,
                &hashed_password,
                &form.profile_picture,
                &form.phone_number,
                &form.is_verified,
                &form.last_login,
                &user_id,
            ],
        )
        .await
    {
        Ok(rows_updated) if rows_updated > 0 => {
            info!("Successfully updated profile with id={}", user_id);
            HttpResponse::Ok().body("User updated")
        }
        Ok(_) => {
            warn!("No profile found with id={}", user_id);
            HttpResponse::NotFound().body(format!("User '{}' not found", user_id))
        }
        Err(err) => {
            error!("failed to update profile with id={}: {:?}", user_id, err);
            HttpResponse::InternalServerError().body("Failed to update profile")
        }
    }
}

pub async fn delete_profile(client: Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    debug!("Deleting user profile");
    let user_id = verify_jwt(&req).expect("oi");
    let query: &str = "DELETE FROM users WHERE id = $1;";
    match client.lock().await.execute(query, &[&user_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            info!("Successfully deleted profile");
            HttpResponse::Ok().body("Deleted user profile")
        }
        Ok(_) => HttpResponse::NotFound().body(format!("User '{}' not found", user_id)),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("User profile not found {}", err))
        }
    }
}
