use std::error::Error;

use async_trait::async_trait;
use crate::{
    net::handler::MessageHandler, protos::gen::raft::Heartbeat, ArcClient, BoxMessageDyn
};

pub struct HeartbeatHandler;

#[async_trait]
impl MessageHandler for HeartbeatHandler {
    async fn handle(&self, msg: BoxMessageDyn, client: &ArcClient) -> Result<(), Box<dyn Error>> {
        let message = msg.downcast_ref::<Heartbeat>().unwrap();
        println!("hey server got heartbeat {:?}", message);
        Ok(())
    }
}
