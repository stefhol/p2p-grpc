use anyhow::Error;
use rusqlite::{Connection, params};
#[allow(unused)]
use uuid::Uuid;

use crate::model::User;




pub fn get_user_db(conn: &Connection,user_name :Option<String>) -> Result<Vec<User>, Error> {
    if user_name.is_some() {
        let mut stmt_user = conn.prepare("SELECT id, name, room_id FROM user Where user = (?1)")?;
        let v = stmt_user.query_map([user_name.unwrap_or_default()], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                room_id: row.get(2)?,
            })
        })?;
        return Ok(v.map(|f| f.unwrap()).collect());
    }
    let mut stmt_user = conn.prepare("SELECT id, name, room_id FROM user ")?;
    let v = stmt_user.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                room_id: row.get(2)?,
            })
        })?;
        Ok(v.map(|f| f.unwrap()).collect())
}

pub fn create_room(conn: &Connection, room_name: &str) -> Result<(), Error> {
    conn.execute("INSERT INTO room (name) VALUES (?1)", params![room_name])?;
    Ok(())
}
pub fn enter_room(conn: &Connection, user: &User, room_name: &str) -> Result<(), Error> {
    //get room id
    let mut stmt = conn.prepare("SELECT (id) FROM room WHERE name = (?1)")?;
    let room_id = stmt.query_row(params![room_name], |row| -> Result<u32, rusqlite::Error> {
        Ok(row.get(0)?)
    })?;
    conn.execute(
        "
    UPDATE user
    SET room_id = (?1)
    WHERE id = (?2);
    ",
        params![room_id, user.id],
    )?;
    Ok(())
}
pub fn leave_room(conn: &Connection, user: &User) -> Result<(), Error> {
    conn.execute(
        "
    UPDATE user
    SET room_id = NULL
    WHERE id = (?1);
    ",
        params![user.id],
    )?;
    Ok(())
}
pub fn add_new_user(conn: &Connection, user: &User) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO user (id, name) VALUES (?1, ?2)",
        params![user.id, user.name],
    )?;
    Ok(())
}

#[test]
fn test_db() {
    let user1 = User {
        id: Uuid::new_v4().to_string(),
        name: "test1".to_string(),
        room_id: None,
    };
    let user2 = User {
        id: Uuid::new_v4().to_string(),
        name: "test2".to_string(),
        room_id: None,
    };
    let conn = crate::db::setup_db().expect("Connection Error");

    println!("Empty DB: {:?}", get_user_db(&conn,None).unwrap());
    add_new_user(&conn, &user1).expect("cant create User");
    add_new_user(&conn, &user2).expect("cant create User");

    let room_name = "room_name";
    create_room(&conn, room_name).unwrap();
    enter_room(&conn, &user1, room_name).unwrap();
    enter_room(&conn, &user2, room_name).unwrap();
    println!("2 users: {:?}", get_user_db(&conn,None).unwrap());
    leave_room(&conn, &user1).unwrap();
    println!("2 users: {:?}", get_user_db(&conn,None).unwrap());
}

