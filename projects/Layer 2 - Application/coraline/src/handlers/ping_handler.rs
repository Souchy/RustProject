use std::error::Error;

use async_trait::async_trait;
use teal::{net::handler::MessageHandler, protos::gen::ping::Ping, ArcClient, BoxMessageDyn};

pub(crate) struct PingHandler;

#[async_trait]
impl MessageHandler for PingHandler {
    async fn handle(&self, msg: BoxMessageDyn, client: &ArcClient) -> Result<(), Box<dyn Error>> {
        let message = msg.downcast_ref::<Ping>().unwrap();
        println!("hey client got ping data {:?}", message);
        Ok(())
    }
}
