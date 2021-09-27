mod chat;
mod db;
pub mod model;
use std::net::SocketAddr;

use actix::Actor;
use anyhow::Error;
use chat::MyChatRoom;
use db::db_actor::Db;
use tonic;
pub mod test {
    tonic::include_proto!("test");
}

use log::{debug, error, info, trace, warn};

use tonic::transport::Server;

use crate::test::chat_room_server::ChatRoomServer;

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let db = Db::new(db::setup_db().expect("error setup db")).start();
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let chat = MyChatRoom::new(db);

    info!(
        "Starting server at localip {}:{} ",
        local_ip_address::local_ip().unwrap().to_string(),
        addr.port()
    );
    Server::builder()
        .add_service(ChatRoomServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
