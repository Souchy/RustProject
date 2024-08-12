use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use realm_commons::protos::client::Identify;
use std::error::Error;
use teal::{net::handler::MessageHandler, DynamicClient};

#[derive(Debug, Default)]
pub(crate) struct IdentifyHandler;
#[async_trait]
impl MessageHandler for IdentifyHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<Identify>().unwrap();
        println!("hey coral got {:?}", message);

        let player_id = message.player_id;
        client.set_id(player_id.clone()).await?;

        Ok(())
    }
}
