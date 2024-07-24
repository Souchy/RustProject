use std::error::Error;

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use redis::Commands;
use teal::{
    net::handler::MessageHandler, protos::messages::RaftHeartbeat, DynamicClient
};

#[derive(Debug, Default)]
pub(crate) struct RaftHeartbeatHandler;
#[async_trait]
impl MessageHandler for RaftHeartbeatHandler {
    async fn handle(&self, msg: DynamicMessage, _client: &DynamicClient) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<RaftHeartbeat>().unwrap();
        println!("hey coral got {:?}", message);
        
        unsafe {
            if let Some(db) = &mut crate::DB {
                db.set("raft_heartbeat", message.leader)?;
            }
        }

		Ok(())
    }
}
