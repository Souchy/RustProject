use std::error::Error;

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use teal::{
    net::handler::MessageHandler,
    protos::messages::Ping,
    DynamicClient,
};

#[derive(Debug, Default)]
pub(crate) struct PingHandler;
#[async_trait]
impl MessageHandler for PingHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error>> {
        let message = msg.transcode_to::<Ping>();
        println!("hey coraline got ping {:?}", message);
        Ok(())
    }
}
