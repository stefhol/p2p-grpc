use anyhow::Error;
use rusqlite::Connection;

mod db;
pub mod db_actor;

pub fn setup_db() -> Result<Connection,Error> {
    let conn = Connection::open_in_memory().expect("Cant open DB in memmory");

        conn.execute(
            "CREATE TABLE user (
                  id              TEXT PRIMARY KEY,
                  room_id          INTEGER,
                  name            TEXT
                  )",
            [],
        )
        .expect("Cant create User table");
        conn.execute(
            "CREATE TABLE room (
                  id              INTEGER PRIMARY KEY,
                  name             TEXT UNIQUE
                  )",
            [],
        )
        .expect("Cant create room table");
        Ok(conn)
}
