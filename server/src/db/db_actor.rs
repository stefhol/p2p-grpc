use actix::{Actor, Context, Handler};
use anyhow::Error;
use rusqlite::{Connection};
use crate::model::User;

use super::db::{add_new_user, create_room, get_user_db};
pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new(conn: Connection) -> Self { Self { conn } }
}
impl Actor for Db {
    type Context = Context<Self>;
    
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }
}
#[derive(Debug)]
pub struct GetUser(pub Option<String>);

impl actix::Message for GetUser {
    type Result = Result<Vec<User>, Error>;
}

impl Handler<GetUser> for Db {
    type Result = Result<Vec<User>, Error>;

    fn handle(&mut self, msg: GetUser, ctx: &mut Self::Context) -> Self::Result {
        get_user_db(&self.conn, msg.0)
    }
}

#[derive(Debug)]
pub struct CreateRoom(String);

impl actix::Message for CreateRoom {
    type Result = Result<(), Error>;
}
impl Handler<CreateRoom> for Db {
    fn handle(&mut self, msg: CreateRoom, ctx: &mut Self::Context) -> Self::Result {
        create_room(&self.conn, &msg.0)
    }

    type Result = Result<(), Error>;
}

#[derive(Debug)]
pub struct AddUser(pub User);

impl actix::Message for AddUser {
    type Result = Result<(), Error>;
}
impl Handler<AddUser> for Db {
    fn handle(&mut self, msg: AddUser, ctx: &mut Self::Context) -> Self::Result {
        add_new_user(&self.conn, &msg.0)
    }

    type Result = Result<(), Error>;
}
