
use rusqlite::{Connection, Result};

pub fn setup() -> Result<()> {
    let conn = Connection::open("crackers.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}