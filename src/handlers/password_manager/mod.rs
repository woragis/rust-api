use std::sync::Arc;

use actix_web::{
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use log::info;
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{
    models::password_manager::user_data::{CreateData, ServiceData, UpdateData},
    shared::types::Id,
    utils::{
        jwt::verify_jwt,
        password_manager::{decrypt, encrypt, generate_key},
    },
};

const TABLE: &str = "password_manager";

pub async fn get_data(client: Data<Arc<Mutex<Client>>>, req: HttpRequest) -> impl Responder {
    let client_id = verify_jwt(&req).expect("ho");
    let query = format!("SELECT * FROM {} WHERE user_id = $1;", TABLE);
    match client.lock().await.query(&query, &[&client_id]).await {
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

pub async fn insert_data(
    client: Data<Arc<Mutex<Client>>>,
    req: HttpRequest,
    data: Json<CreateData>,
) -> impl Responder {
    let user_id = verify_jwt(&req).expect("hi");
    let query = format!(
        "INSERT INTO {} 
    name, email, username, password, user_id
    VALUES ($1, $2, $3, $4, $5);",
        TABLE
    );
    match client
        .lock()
        .await
        .execute(
            &query,
            &[
                &data.name,
                &data.email,
                &data.username,
                &data.password,
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
    let user_id = verify_jwt(&req).expect("hi");
    let query = format!("UPDATE {} SET name = $1, email = $2, username = $3, password = $4 WHERE id = $5 AND user_id = $6;", TABLE);
    match client
        .lock()
        .await
        .execute(
            &query,
            &[
                &data.name,
                &data.email,
                &data.username,
                &data.password,
                &data.id,
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
    let user_id = verify_jwt(&req).expect("ho");
    let query = format!("SELECT * FROM {} WHERE user_id = $1;", TABLE);
    match client.lock().await.query(&query, &[&*data_id]).await {
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
    let user_id = verify_jwt(&req).expect("ho");
    let query = format!("DELETE FROM {} WHERE id = $1;", TABLE);
    match client.lock().await.execute(&query, &[&*data_id]).await {
        Ok(_) => {
            info!("Succeffully deleted data from password manager app");
            HttpResponse::Ok().body("Deleted data")
        }
        Err(_) => HttpResponse::InternalServerError().body("hi"),
    }
}
