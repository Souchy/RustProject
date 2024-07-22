use std::error::Error;

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use teal::{
    net::handler::MessageHandler, protos::messages::RaftHeartbeat, DynamicClient
};

#[derive(Debug, Default)]
pub(crate) struct RaftHeartbeatHandler;
#[async_trait]
impl MessageHandler for RaftHeartbeatHandler {
    async fn handle(&self, msg: DynamicMessage, _client: &DynamicClient) -> Result<(), Box<dyn Error>> {
        let message = msg.transcode_to::<RaftHeartbeat>();
        println!("hey coral got RaftHeartbeat {:?}", message);
		Ok(())
    }
}
