use rusqlite::{Connection, Result};

fn init_users(conn: &Connection) -> Result<()> {
    let sql_query = "
      CREATE TABLE IF NOT EXISTS user (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      age INTEGER
    );";
    conn.execute(sql_query, [])?;
    Ok(())
}

fn init_notes(conn: &Connection) -> Result<()> {
    let sql_query: &str = "
      CREATE TABLE IF NOT EXISTS notes (
      id INTEGER PRIMARY KEY,
      title TEXT NOT NULL,
      body TEXT,
      created_at TIMESTAMPTZ
    );";
    conn.execute(sql_query, [])?;
    Ok(())
}

pub fn init_db() -> Result<()> {
    let conn = Connection::open("mydatabase.db")?;

    let users = init_users(&conn);
    let notes = init_notes(&conn);
    println!("Database connected!");
    Ok(())
}
