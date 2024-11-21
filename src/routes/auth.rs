use crate::db::connection::DbConnection;
use crate::models::user::{LoginRequest, User};
use crate::utils::jwt::create_jwt_token;
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};

#[post("/auth/login")]
async fn login(
    db: web::Data<DbConnection>,
    credentials: web::Json<LoginRequest>,
) -> impl Responder {
    let client = db.get_client();

    let user = match User::find_by_email(client, &credentials.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid email or password"),
        Err(err) => return HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    };

    let argon2 = Argon2::default();

    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(parsed) => parsed,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid email or password"),
    };

    if let Err(_) = argon2.verify_password(credentials.password.as_bytes(), &parsed_hash) {
        return HttpResponse::Unauthorized().body("Invalid email or password");
    }

    match create_jwt_token(&user.email, user.id) {
        Ok(token) => HttpResponse::Ok().json(web::Json(token)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
