use std::sync::Arc;

use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use log::info;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{
    config::encryption::{BLOCK_SIZE, KEY}, db::tables::password_manager::PASSWORD_MANAGER_TABLE, models::{password_manager::user_data::{CreateData, ServiceData, UpdateData}, user::UserId}, shared::types::Id, utils::jwt::verify_jwt
};

pub async fn get_data(client: Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    let user_id: UserId = verify_jwt(&req).expect("ho");
    let stmt: String = format!("SELECT * FROM {} WHERE user_id = $1;", PASSWORD_MANAGER_TABLE);
    match client.lock().await.query(&stmt, &[&user_id]).await {
        Ok(rows) => {
            let user_saved_data: Vec<ServiceData> = rows
                .into_iter()
                .map(|row| ServiceData::decrypt_row(row, KEY))
                .collect();
            info!("Successfully retrieved all user data from password manager app");
            // HttpResponse::Ok().json(articles)
            HttpResponse::Ok().json(user_saved_data)
        }
        Err(_) => HttpResponse::InternalServerError().body("hi"),
    }
}

pub async fn insert_data(
    client: Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
    data: Json<CreateData>,
) -> impl Responder {
    let user_id: UserId = verify_jwt(&req).expect("hi");
    let encrypted_data: CreateData = CreateData::encrypt_data(KEY, data, BLOCK_SIZE);
    let stmt: String = format!(
        "INSERT INTO {} 
        name, email, username, password, user_id
        VALUES ($1, $2, $3, $4, $5);",
        PASSWORD_MANAGER_TABLE
    );
    match client
        .lock()
        .await
        .execute(
            &stmt,
            &[
                &encrypted_data.name,
                &encrypted_data.email,
                &encrypted_data.username,
                &encrypted_data.password,
                &user_id,
            ],
        )
        .await
    {
        Ok(_) => HttpResponse::Created().body("Successfully saved your data"),
        Err(_) => HttpResponse::InternalServerError().body("hi"),
    }
}

pub async fn update_data(
    client: Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
    data: Json<UpdateData>,
) -> impl Responder {
    let user_id: UserId = verify_jwt(&req).expect("hi");
    let encrypted_data: UpdateData = UpdateData::encrypt_data(KEY, data, BLOCK_SIZE);
    let stmt: String = format!("UPDATE {} SET name = $1, email = $2, username = $3, password = $4 WHERE id = $5 AND user_id = $6;", PASSWORD_MANAGER_TABLE);
    match client
        .lock()
        .await
        .execute(
            &stmt,
            &[
                &encrypted_data.name,
                &encrypted_data.email,
                &encrypted_data.username,
                &encrypted_data.password,
                &encrypted_data.id,
                &user_id,
            ],
        )
        .await
    {
        Ok(_) => HttpResponse::Created().body("Successfully saved your data"),
        Err(_) => HttpResponse::InternalServerError().body("hi"),
    }
}

pub async fn get_single_data(
    client: Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
    data_id: Path<Id>,
) -> impl Responder {
    let user_id: UserId = verify_jwt(&req).expect("ho");
    let stmt: String = format!("SELECT * FROM {} WHERE id = $1 AND user_id = $2;", PASSWORD_MANAGER_TABLE);
    match client
        .lock()
        .await
        .query(&stmt, &[&*data_id, &user_id])
        .await
    {
        Ok(rows) => {
            let user_saved_data: Vec<ServiceData> = rows
                .into_iter()
                .map(|row| ServiceData::from_row(row))
                .collect();
            info!("Successfully retrieved all user data from password manager app");
            // HttpResponse::Ok().json(articles)
            HttpResponse::Ok().json(user_saved_data)
        }
        Err(_) => HttpResponse::InternalServerError().body("hi"),
    }
}

pub async fn delete_data(
    client: Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
    data_id: Path<Id>,
) -> impl Responder {
    let user_id: UserId = verify_jwt(&req).expect("ho");
    let stmt: String = format!("DELETE FROM {} WHERE id = $1 AND user_id = $2;", PASSWORD_MANAGER_TABLE);
    match client
        .lock()
        .await
        .execute(&stmt, &[&*data_id, &user_id])
        .await
    {
        Ok(_) => {
            info!("Succeffully deleted data from password manager app");
            HttpResponse::Ok().body("Deleted data")
        }
        Err(_) => HttpResponse::InternalServerError().body("hi"),
    }
}
