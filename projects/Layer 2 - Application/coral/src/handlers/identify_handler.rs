use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use realm_commons::protos::client::Identify;
use teal::{
    net::{client::DefaultClient, handler::MessageHandler},
    DynamicClient,
};

#[derive(Debug, Default)]
pub(crate) struct IdentifyHandler;
#[async_trait]
impl MessageHandler for IdentifyHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        _client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<Identify>().unwrap();
        println!("hey coral got {:?}", message);

        // let ser = _client.get_server();
        // let mut server = ser.lock().await;

        // let id = _client.get_id_ref().lock().await.clone();
        // server.clients.remove(&id);

        let player_id = message.player_id;
        _client.set_id(player_id.clone()).await?;

        // let c = _client.downcast_ref::<DefaultClient>().unwrap().clone();
        // server.clients.insert(player_id.clone(), Arc::new(c));

        Ok(())
    }
}
