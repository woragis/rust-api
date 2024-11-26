use crate::models::order::{CreateOrderRequest, Order};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_order(
    client: web::Data<Arc<Mutex<Client>>>,
    order: web::Json<CreateOrderRequest>,
) -> impl Responder {
    println!("Creating Order");
    let query = "INSERT INTO orders (
        user_id, status, total_amount) VALUES (
        $1, $2, $3) RETURNING *";
    match client
        .lock()
        .await
        .query_one(query, &[&order.user_id, &order.status, &order.total_amount])
        .await
    {
        Ok(row) => {
            let id = row.get("id");
            println!("Created Order: '{}'", id);
            HttpResponse::Created().json(Order {
                id,
                user_id: row.get("user_id"),
                order_date: row.get("order_date"),
                status: row.get("status"),
                total_amount: row.get("total_amount"),
            })
        }
        Err(err) => {
            eprintln!("Failed to create order: {}", err);
            HttpResponse::InternalServerError().body("Failed to create order")
        }
    }
}

pub async fn read_order(
    client: web::Data<Arc<Mutex<Client>>>,
    order_id: web::Path<i32>,
) -> impl Responder {
    println!("Reading Order '{}'", order_id);
    let query = "SELECT * FROM orders WHERE id = $1";
    match client.lock().await.query_one(query, &[&*order_id]).await {
        Ok(row) => {
            let order = Order {
                id: row.get("id"),
                user_id: row.get("user_id"),
                order_date: row.get("order_date"),
                status: row.get("status"),
                total_amount: row.get("total_amount"),
            };
            println!("Read Order '{}'", order.id);
            HttpResponse::Ok().json(order)
        }
        Err(err) => {
            eprintln!("Order not found: {}", err);
            HttpResponse::NotFound().body("Order not found")
        }
    }
}

pub async fn read_orders(client: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    println!("Reading Orders");
    let query = "SELECT * FROM orders";
    match client.lock().await.query(query, &[]).await {
        Ok(rows) => {
            let orders: Vec<Order> = rows
                .into_iter()
                .map(|row| Order {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    order_date: row.get("order_date"),
                    status: row.get("status"),
                    total_amount: row.get("total_amount"),
                })
                .collect();
            println!("Read Orders");
            HttpResponse::Ok().json(orders)
        }
        Err(err) => {
            eprintln!("Error fetching orders: {}", err);
            HttpResponse::InternalServerError().body("Failed to fetch orders")
        }
    }
}

/*
pub async fn update_order(
    client: web::Data<Arc<Mutex<Client>>>,
    order_id: web::Path<i32>,
    order: web::Json<UpdateProductRequest>,
) -> impl Responder {
    println!("Updating order '{}'", order_id);
    let query = "UPDATE orders SET
        user_id = $1, order_date = $2,
        status = $3, total_amount = $4 WHERE id = $5);";
    match client
        .lock()
        .await
        .execute(
            query,
            &[
                &order.user_id,
                &order.order_date,
                &order.status,
                &order.total_amount,
                &*order.id,
            ],
        )
        .await
    {
        Ok(rows_updated) if rows_updated > 0 => {
            println!("Updated order '{}'", order_id);
            HttpResponse::Ok().body("Order updated")
        }
        Ok(_) => HttpResponse::NotFound().body(format!("Order '{}' not found", order_id)),
        Err(err) => {
            eprintln!("Failed to update order: {}", err);
            HttpResponse::InternalServerError().body("Failed to update order")
        }
    }
}
*/

pub async fn delete_order(
    client: web::Data<Arc<Mutex<Client>>>,
    order_id: web::Path<i32>,
) -> impl Responder {
    println!("Deleting Order '{}'", order_id);
    let query = "DELETE FROM orders WHERE id = $1";
    match client.lock().await.execute(query, &[&*order_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            println!("Deleted Order '{}'", order_id);
            HttpResponse::Ok().body("Order deleted")
        }
        Ok(_) => HttpResponse::NotFound().body(format!("Order '{}' not found", order_id)),
        Err(err) => {
            eprintln!("Failed to delete order: {}", err);
            HttpResponse::InternalServerError().body("Failed to delete order")
        }
    }
}
