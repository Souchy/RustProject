use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use teal::{
    net::{client::Client, handler::MessageHandler, server::Server},
    protos::messages::RaftHeartbeat,
    DynamicClient,
};
use tokio::sync::Mutex;

#[derive(Clone, Default, Debug)]
pub struct StubClient {
    pub id: Arc<Mutex<String>>
}
impl StubClient {
    pub fn new() -> Self {
        Self {
            id: Arc::new(Mutex::new(String::from("3")))
        }
    }
}

#[async_trait]
impl Client for StubClient {
    fn get_id_ref(&self) -> Arc<Mutex<String>> {
        self.id.clone()
    }
    fn get_id_sync(&self) -> String {
        self.id.blocking_lock().clone()
    }
    async fn set_id(&self, id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        *self.id.lock().await = id;
        Ok(())
    }
    fn get_server(&self) -> Arc<Mutex<Server>> {
        Arc::new(Mutex::new(Server::default()))
    }
    async fn send_bytes(&self, _buf: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
    async fn run(&self) -> Result<(), Box<dyn Error + Send>> {
        Ok(())
    }
    async fn frame(&self, _buf: &[u8]) {}
    // async fn send<T: MessageIdentifiable + MessageFull>(
    //     &self,
    //     msg: T,
    // ) -> Result<(), Box<dyn Error>> {
    //     Ok(())
    // }
    // async fn broadcast<T: MessageIdentifiable + MessageFull>(&mut self, msg: T) {}
}

#[derive(Debug, Default)]
pub struct RaftHeartbeatHandlerAssertTerm4;
#[async_trait]
impl MessageHandler for RaftHeartbeatHandlerAssertTerm4 {
    async fn handle(
        &self,
        msg: DynamicMessage,
        _client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<RaftHeartbeat>().unwrap();
        assert_eq!(message.term, 4);
        Ok(())
    }
}
