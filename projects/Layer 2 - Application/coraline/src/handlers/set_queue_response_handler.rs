use std::error::Error;

use async_trait::async_trait;
use coral_commons::protos::messages::SetQueueResponse;
use prost_reflect::DynamicMessage;
use teal::{
    net::handler::MessageHandler,
    DynamicClient,
};

#[derive(Debug, Default)]
pub(crate) struct SetQueueResponseHandler;
#[async_trait]
impl MessageHandler for SetQueueResponseHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        _client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<SetQueueResponse>().unwrap();
        println!("hey coraline got {:?}", message);

		Ok(())
    }
}
