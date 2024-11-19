use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;
use uuid::Uuid;

mod db;
use db::{delete_item, get_all_items, get_item_by_id, init_db, insert_item, update_item, Item};

#[derive(Clone)]
struct AppState {
    db_client: Arc<Mutex<Client>>,
}

async fn create_item(item: web::Json<Item>, state: web::Data<AppState>) -> impl Responder {
    let client = state.db_client.lock().unwrap();
    let new_item = Item::new(item.name.clone());

    match insert_item(&client, &new_item).await {
        Ok(_) => HttpResponse::Created().json(new_item),
        Err(_) => HttpResponse::InternalServerError().body("Failed to insert item"),
    }
}

async fn get_all_items_handler(state: web::Data<AppState>) -> impl Responder {
    let client = state.db_client.lock().unwrap();
    match get_all_items(&client).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch items"),
    }
}

async fn get_item_handler(item_id: web::Path<Uuid>, state: web::Data<AppState>) -> impl Responder {
    let client = state.db_client.lock().unwrap();
    match get_item_by_id(&client, item_id.into_inner()).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().body("Item not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch item"),
    }
}

async fn update_item_handler(
    item_id: web::Path<Uuid>,
    item: web::Json<Item>,
    state: web::Data<AppState>,
) -> impl Responder {
    let client = state.db_client.lock().unwrap();
    match update_item(&client, item_id.into_inner(), item.name.clone()).await {
        Ok(_) => HttpResponse::Ok().body("Item updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update item"),
    }
}

async fn delete_item_handler(
    item_id: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> impl Responder {
    let client = state.db_client.lock().unwrap();
    match delete_item(&client, item_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete item"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Program is running!");
    let client = init_db().await.unwrap();
    let state = web::Data::new(AppState {
        db_client: Arc::new(Mutex::new(client)),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .route("/items", web::post().to(create_item))
            .route("/items", web::get().to(get_all_items_handler))
            .route("/items/{id}", web::get().to(get_item_handler))
            .route("/items/{id}", web::put().to(update_item_handler))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
