use crate::config::jwt::SECRET;
use crate::models::auth::Claims;
use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error, info, warn};

pub fn create_jwt(user_id: i32, user_email: String) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.clone(),
        email: user_email.clone(),
        exp: expiration,
    };

    debug!(
        "Creating JWT for user_id={} with email={} and expiration={}",
        user_id, user_email, expiration
    );

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .expect("JWT encoding should succeed");

    info!("JWT successfully created for user_id={}", user_id);

    token
}

pub fn verify_jwt(req: &HttpRequest) -> Option<i32> {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        debug!("Authorization header found: {:?}", header_value);

        if let Ok(auth_str) = header_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Strip "Bearer " prefix
                debug!("Extracted token: {}", token);

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(SECRET),
                    &Validation::default(),
                );

                match token_data {
                    Ok(data) => {
                        // Return the user ID
                        info!("JWT Successfully verified for user_id={}", data.claims.sub);
                        Some(data.claims.sub)
                    }
                    Err(err) => {
                        error!("JWT Verification failed: {:?}", err);
                        HttpResponse::Unauthorized().body("Invalid token");
                        None
                    }
                }
            } else {
                warn!("Authorization format invalid, expected 'Bearer <token>'");
                HttpResponse::Unauthorized().body("Invalid authorization format");
                None
            }
        } else {
            error!("Failed to parse Authentication header");
            HttpResponse::Unauthorized().body("Invalid authorization header");
            None
        }
    } else {
        warn!("Authorization header is missing");
        HttpResponse::Unauthorized().body("Authorization header missing");
        None
    }
}
