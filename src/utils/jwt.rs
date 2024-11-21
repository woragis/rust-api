use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use std::env;

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,
    user_id: i32,
    exp: usize,
}

pub fn create_jwt_token(email: &str, user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
