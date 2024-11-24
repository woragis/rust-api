use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub async fn create_products_table(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    let create_users_table_sql = "
        CREATE TABLE IF NOT EXISTS products (
        id bigserial primary key,
        name varchar(255) not null,
        description text,
        category varchar(100),
        images text[],
        price decimal(10, 2) not null,
        discount decimal(5, 2) default 0.00,
        currency varchar(3) default 'usd',
        stock int default 0,
        weight decimal(10, 2),
        dimensions jsonb,
        tags text[],
        is_active boolean default true,
        created_at timestamp default current_timestamp,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );";
    client
        .lock()
        .await
        .execute(create_users_table_sql, &[])
        .await?;
    Ok(())
}
