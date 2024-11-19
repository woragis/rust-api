use tokio_postgres::{Client, Error};

pub async fn create_table(client: &Client) -> Result<(), Error> {
    let query = "
    CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
    );
  ";
    client.execute(query, &[]).await?;

    println!("Table 'users' created succeffully");

    Ok(())
}

// Create (Insert)
pub async fn create_user(client: &Client, name: &str) -> Result<(), tokio_postgres::Error> {
    let query = "INSERT INTO users (name) VALUES ($1)";
    client.execute(query, &[&name]).await?;
    println!("User '{}' added successfully.", name);
    Ok(())
}

// Read (Select)
pub async fn read_users(client: &Client) -> Result<(), tokio_postgres::Error> {
    let query = "SELECT id, name FROM users";
    let rows = client.query(query, &[]).await?;
    println!("Current users:");
    for row in rows {
        let id: i32 = row.get("id");
        let name: &str = row.get("name");
        println!("  id: {}, name: {}", id, name);
    }
    Ok(())
}

// Update
pub async fn update_user(
    client: &Client,
    id: i32,
    new_name: &str,
) -> Result<(), tokio_postgres::Error> {
    let query = "UPDATE users SET name = $1 WHERE id = $2";
    let updated = client.execute(query, &[&new_name, &id]).await?;
    println!("Updated {} user(s).", updated);
    Ok(())
}

// Delete
pub async fn delete_user(client: &Client, id: i32) -> Result<(), tokio_postgres::Error> {
    let query = "DELETE FROM users WHERE id = $1";
    let deleted = client.execute(query, &[&id]).await?;
    println!("Deleted {} user(s).", deleted);
    Ok(())
}
