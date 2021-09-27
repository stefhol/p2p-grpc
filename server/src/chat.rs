use crate::db::db_actor::{AddUser, Db, GetUser};

use crate::model::User;
use crate::test::chat_room_server::ChatRoom;
use crate::test::{self, *};

use actix::Addr;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;



pub struct MyChatRoom {
    db_actor: Addr<Db>,
}

impl MyChatRoom {
    pub fn new(db_actor: Addr<Db>) -> Self {
        Self { db_actor }
    }
}

#[tonic::async_trait]
impl ChatRoom for MyChatRoom {
    ///////////
    ///Hello test
    //////////
    async fn say_hello(
        &self,
        request: Request<NameRequest>,
    ) -> Result<Response<NameReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = test::NameReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
    ///////////
    ///Add user
    //////////
    async fn add_user(
        &self,
        request: tonic::Request<test::NameRequest>,
    ) -> Result<tonic::Response<test::StatusResponse>, tonic::Status> {
        let res = self
            .db_actor
            .send(AddUser(User {
                id: Uuid::new_v4().to_string(),
                name: request.into_inner().name,
                room_id: None,
            }))
            .await;
        if res.is_err() {
            return Ok(Response::new(test::StatusResponse { sucess: false }));
        };
        Ok(Response::new(test::StatusResponse { sucess: true }))
    }
    ///////////
    ///get user
    /////////

    type get_userStream = ReceiverStream<Result<NameReply, Status>>;

    async fn get_user(
        &self,
        request: tonic::Request<test::OptionalNameRequest>,
    ) -> Result<tonic::Response<Self::get_userStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(4);
        let name = request.into_inner().name;
        let res = self.db_actor.send(GetUser(name)).await;
        tokio::spawn(async move {
            if res.is_ok() {
                let res = res.unwrap();
                if res.is_ok() {
                    let res = res.unwrap();

                    for user in res {
                        tx.send(Ok(NameReply { message: user.name })).await.unwrap();
                    }
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    ///////////    
    ///delete user
    /////////
    async fn delete_user(
        &self,
        request: tonic::Request<test::NameRequest>,
    ) -> Result<tonic::Response<test::StatusResponse>, tonic::Status> {
        todo!()
    }
    type join_roomStream = ReceiverStream<Result<NameReply, Status>>;
    ///////////    
    ///join room
    /////////
    async fn join_room(
        &self,
        request: tonic::Request<test::NameRequest>,
    ) -> Result<tonic::Response<Self::join_roomStream>, tonic::Status> {
        todo!()
    }

     ///////////    
    ///leave room
    /////////
    async fn leave_room(
        &self,
        request: tonic::Request<test::NameRequest>,
    ) -> Result<tonic::Response<test::StatusResponse>, tonic::Status> {
        todo!()
    }


}
