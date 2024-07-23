use std::error::Error;

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use realm_commons::protos::client::Identify;
use teal::{net::handler::MessageHandler, DynamicClient};

#[derive(Debug, Default)]
pub(crate) struct IdentifyHandler;
#[async_trait]
impl MessageHandler for IdentifyHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        _client: &DynamicClient,
    ) -> Result<(), Box<dyn Error>> {
        let message = msg.transcode_to::<Identify>().unwrap();
        println!("hey coral got {:?}", message);

		_client.set_id(message.player_id).await
    }
}
