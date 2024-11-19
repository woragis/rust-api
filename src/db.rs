use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, NoTls};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub created_at: String,
}

impl Item {
    pub fn new(name: String) -> Self {
        Item {
            id: Uuid::new_v4(),
            name,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

pub async fn init_db() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=mysecretpassword dbname=actix_crud",
        NoTls,
    )
    .await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error while connecting: {}", e);
        }
    });
    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS items (
      id UUID PRIMARY KEY,
      name TEXT NOT NULL,
      created_at TIMESTAMPTZ NOT NULL,
  );",
        )
        .await?;
    Ok(client)
}

pub async fn insert_item(client: &Client, item: &Item) -> Result<(), Error> {
    let stmt = "INSERT INTO items (id, name, created_at) VALUES ($1, $2, $3);";
    client
        .execute(stmt, &[&item.id.to_string(), &item.name, &item.created_at])
        .await?;

    Ok(())
}

pub async fn get_all_items(client: &Client) -> Result<Vec<Item>, Error> {
    let stmt = "SELECT id, name, created_at FROM items;";
    let rows = client.query(stmt, &[]).await?;
    let mut items = Vec::new();
    for row in rows {
        let item = Item {
            id: row.get::<Uuid, _>(0),
            name: row.get(1),
            created_at: row.get(2),
        };
        items.push(item);
    }

    Ok(items)
}

/*
pub async fn get_item_by_id(client: &Client, id: Uuid) -> Result<Option<Item>, Error> {
    let stmt = "SELECT id, name, created_at FROM items WHERE id=$1";
    let rows = client.query(stmt, &[&id]).await?;

    if let Some(row) = rows.into_iter().next() {
        let item = Item {
            id: row.get(0),
            name: row.get(1),
            created_at: row.get(2),
        };
        Ok(Some(item))
    } else {
        Ok(None)
    }
}

pub async fn update_item(client: &Client, id: Uuid, name: String) -> Result<(), Error> {
    let stmt = "UPDATE INTO items SET name=$1 WHERE id=$2";
    client.query(stmt, &[&name, &id.to_string()]).await?;
    Ok(())
}

pub async fn delete_item(client: &Client, id: Uuid) -> Result<(), Error> {
    let stmt = "DELETE FROM items WHERE id=$1";
    client.execute(stmt, &[&id.to_string()]).await?;

    Ok(())
}

*/