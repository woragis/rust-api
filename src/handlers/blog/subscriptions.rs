use std::sync::Arc;

use actix_web::{web::{Data, Json}, HttpRequest, HttpResponse, Responder};
use log::{info, error, debug};
use tokio::sync::Mutex;
use tokio_postgres::Client;

use crate::{db::tables::blog::SUBSCRIPTION_TABLE, models::blog::subscription::SubscribeRequest, utils::jwt::verify_jwt};


pub async fn subscribe(client: Data<Arc<Mutex<Client>>>, req: HttpRequest, body: Json<SubscribeRequest>) -> impl Responder{
    debug!("Subscribing user to a blogger");
    let user_id = verify_jwt(&req).expect("hi");
    let stmt = format!("INSERT INTO {} (user_id, blogger_id) VALUES ($1, $2);", SUBSCRIPTION_TABLE);
    match client.lock().await.execute(&stmt, &[
        &user_id,
        &body.blogger_id
    ]).await {
        Ok(_) => {
            info!("Successfully subscribed user: '{}' to blogger: '{}'", user_id, &body.blogger_id);
            HttpResponse::Created().body("subscribed")
        },
        Err(_) => {
            error!("Failed to subscribe user: '{}' to blogger: '{}'", user_id, &body.blogger_id);
            HttpResponse::InternalServerError().body("could not subscribe you to blogger")
        }
    }
}