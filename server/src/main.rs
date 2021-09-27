mod chat;
mod db;
pub mod model;
use actix::{Actor};
use chat::MyChatRoom;
use db::db_actor::{ Db};
use tonic;
pub mod test {
    tonic::include_proto!("test");
}
use tonic::transport::Server;

use crate::test::chat_room_server::ChatRoomServer;


#[actix_rt::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Db::new(db::setup_db().expect("error setup db")).start();
    let addr = "[::1]:50051".parse()?;
    let chat = MyChatRoom::new(db);

    Server::builder()
        .add_service(ChatRoomServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
