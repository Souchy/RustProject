use std::error::Error;

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use realm_commons::protos::server::CreatedLobby;
use teal::{net::handler::MessageHandler, DynamicClient};

#[derive(Debug, Default)]
pub(crate) struct CreatedLobbyHandler;
#[async_trait]
impl MessageHandler for CreatedLobbyHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<CreatedLobby>().unwrap();
        println!("hey coraline got {:?}", message);

        // Set lobby in active queue
        // let mut queue = SetQueueRequest::default();
        // queue.queue = 1;
        // queue.state = QueueState::Active as i32;
        // queue.lobby = message.lobby;
        // let buf = serialize(&queue);
        // client.send_bytes(&buf).await
        Ok(())
    }
}
