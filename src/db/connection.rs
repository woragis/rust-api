use tokio_postgres::{Client, Error, NoTls};

pub async fn connect_db() -> Result<Client, Error> {
    let connection_str = "host=localhost user=postgres password=yourpassword database=api";
    let (client, connection) = tokio_postgres::connect(connection_str, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let sql_client_query = "SELECT * FROM users;";
    let rows = client.query(sql_client_query, &[]).await?;
    for row in rows {
        let id: u32 = row.get(0);
        let name: &str = row.get(1);
        println!("Found row: id = {}, name = {}", id, name);
    }

    Ok(client)
}
