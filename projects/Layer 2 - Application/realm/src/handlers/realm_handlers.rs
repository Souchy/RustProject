use std::error::Error;

use async_trait::async_trait;
use realm_commons::protos::{
    client::gen::{CreateLobby::CreateLobby, JoinLobby::JoinLobby}, server::gen::CreatedLobby::CreatedLobby,
};
use teal::{
    net::{handler::MessageHandler, message::serialize},
    ArcClient, BoxMessageDyn,
};

pub(crate) struct CreateLobbyHandler;
#[async_trait]
impl MessageHandler for CreateLobbyHandler {
    async fn handle(&self, msg: BoxMessageDyn, client: &ArcClient) -> Result<(), Box<dyn Error>> {
        let message = msg.downcast_ref::<CreateLobby>().unwrap();
        println!("hey realm got CreateLobby {:?}", message);

        // TODO Create lobby in the queue
        let queue = message.queue();
        // create a token for the lobby and set it.
        // set on redis

        let created: CreatedLobby; // = CreatedLobby::new();

        Ok(())
    }
}

pub(crate) struct JoinLobbyHandler;
#[async_trait]
impl MessageHandler for JoinLobbyHandler {
    async fn handle(&self, msg: BoxMessageDyn, client: &ArcClient) -> Result<(), Box<dyn Error>> {
        let message = msg.downcast_ref::<JoinLobby>().unwrap();
        println!("hey realm got JoinLobby {:?}", message);

        // TODO Add the player to the lobby, if the token matches and the lobby isnt full
        let queue = message.queue?;
        let lobby = message.lobby();
        let token = message.token();
        // set on redis

        Ok(())
    }
}
