use std::error::Error;

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use teal::{
    net::{handler::MessageHandler, message::serialize},
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
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<Ping>();
        println!("hey coral got {:?}", message);
        // TODO send
        // return client.send(Ping::new()).await;
        let buf = serialize(&Ping::default());
        return client.send_bytes(&buf).await;
    }
}
