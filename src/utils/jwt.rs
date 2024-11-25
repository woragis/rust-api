use crate::config::jwt::SECRET;
use crate::models::{auth::Claims, user::User};
use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub fn create_jwt(user: &User) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .expect("JWT encoding should succeed")
}

/// Middleware to verify the JWT token
pub fn verify_jwt(req: &HttpRequest) -> Result<u32, HttpResponse> {
    let auth_header = req.headers().get("Authorization");

    if let Some(header_value) = auth_header {
        if let Ok(auth_str) = header_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Strip "Bearer " prefix
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(SECRET),
                    &Validation::default(),
                );

                match token_data {
                    Ok(data) => Ok(data.claims.sub), // Return the user ID
                    Err(_) => Err(HttpResponse::Unauthorized().body("Invalid token")),
                }
            } else {
                Err(HttpResponse::Unauthorized().body("Invalid authorization format"))
            }
        } else {
            Err(HttpResponse::Unauthorized().body("Invalid authorization header"))
        }
    } else {
        Err(HttpResponse::Unauthorized().body("Authorization header missing"))
    }
}
