use std::error::Error;

use async_trait::async_trait;
use coral_commons::protos::models::Match;
use prost_reflect::DynamicMessage;
use teal::{
    net::handler::MessageHandler,
    DynamicClient,
};

#[derive(Debug, Default)]
pub(crate) struct MatchHandler;
#[async_trait]
impl MessageHandler for MatchHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        _client: &DynamicClient,
    ) -> Result<(), Box<dyn Error>> {
        let message = msg.transcode_to::<Match>().unwrap();
        println!("hey coraline got {:?}", message);

		Ok(())
    }
}
