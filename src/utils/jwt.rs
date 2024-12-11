use crate::models::auth::Claims;
use crate::{config::jwt::SECRET, models::user::UserId};
use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error, info, warn};

pub fn create_jwt(user_id: UserId, user_email: String) -> String {
    let expiration: usize = chrono::Utc::now()
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
    .unwrap();

    info!("JWT successfully created for user_id={}", user_id);

    token
}

pub fn verify_jwt(req: &HttpRequest) -> Result<UserId, HttpResponse> {
    let auth_header = req.headers().get("Authorization").ok_or_else(|| {
        warn!("Authorization header is missing");
        HttpResponse::Unauthorized().body("Authorization header missing")
    })?;
    debug!("Authorization header found: {:?}", auth_header);

    let auth_str = auth_header
        .to_str()
        .map_err(|_| {
            error!("Failed to parse Authorization header");
            HttpResponse::Unauthorized().body("Invalid authorization header")
        })
        .expect("oi");

    if !auth_str.starts_with("Bearer ") {
        warn!("Authorization format invalid, expected 'Bearer <token>'");
        return Err(HttpResponse::Unauthorized().body("Invalid authorization format"));
    }

    let token = &auth_str[7..]; // Strip "Bearer " prefix
    debug!("Extracted token: {}", token);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map_err(|err| {
        error!("JWT Verification failed: {:?}", err);
        HttpResponse::Unauthorized().body("Invalid token")
    })?;

    info!(
        "JWT Successfully verified for user_id={}",
        token_data.claims.sub
    );
    Ok(token_data.claims.sub)
}
