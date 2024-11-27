use crate::{
    models::order::{CreateOrderRequest, Order},
    utils::admin::verify_admin,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_order(
    client: web::Data<Arc<Mutex<Client>>>,
    order: web::Json<CreateOrderRequest>,
) -> impl Responder {
    debug!("Inserting new order into the database");
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
    debug!("Inserting new order into the database");
    let query = "SELECT * FROM orders WHERE id = $1";
    match client.lock().await.query_one(query, &[&*order_id]).await {
        Ok(row) => {
            let order = Order::from_row(row);
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
    debug!("Inserting new order into the database");
    let query = "SELECT * FROM orders";
    match client.lock().await.query(query, &[]).await {
        Ok(rows) => {
            let orders: Vec<Order> = rows.into_iter().map(|row| Order::from_row(row)).collect();
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
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for creating a user");
    match verify_admin(&client, &req).await {
        Ok(_) => info!("Admin privileges verified"),
        Err(err) => {
            warn!("Admin verification failed: {:?}", err);
            return HttpResponse::Unauthorized().body("You are not admin");
        }
    };

    debug!("Inserting new order into the database");
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
    req: HttpRequest,
) -> impl Responder {
    debug!("Verifying admin privileges for deleting a order");
    match verify_admin(&client, &req).await {
        Ok(_) => info!("Admin privileges verified"),
        Err(err) => {
            warn!("Admin verification failed: {:?}", err);
            return HttpResponse::Unauthorized().body("You are not admin");
        }
    };

    debug!("Deleting order from database");
    let query = "DELETE FROM orders WHERE id = $1";
    match client.lock().await.execute(query, &[&*order_id]).await {
        Ok(rows_deleted) if rows_deleted > 0 => {
            info!("Successfully deleted order with id={}", order_id);
            HttpResponse::Ok().body("Order deleted")
        }
        Ok(_) => {
            warn!("Failed to find order with id={}", order_id);
            HttpResponse::NotFound().body(format!("Order '{}' not found", order_id))
        }
        Err(err) => {
            error!("Failed to delete order: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to delete order")
        }
    }
}
