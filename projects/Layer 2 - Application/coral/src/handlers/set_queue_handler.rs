use std::error::Error;

use async_trait::async_trait;
use coral_commons::protos::messages::SetQueueRequest;
use prost_reflect::DynamicMessage;
use redis::Commands;
use teal::{net::handler::MessageHandler, DynamicClient};

#[derive(Debug, Default)]
pub(crate) struct SetQueueHandler;
#[async_trait]
impl MessageHandler for SetQueueHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        _client: &DynamicClient,
    ) -> Result<(), Box<dyn Error>> {
        let message = msg.transcode_to::<SetQueueRequest>().unwrap();
        println!("hey coral got SetQueueRequest {:?}", message);

        unsafe {
            if let Some(db) = &mut crate::DB {
                db.set("set_queue", message.queue)?;
            }
        }

        Ok(())
    }
}
