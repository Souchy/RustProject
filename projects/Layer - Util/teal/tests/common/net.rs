use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use teal::{
    net::{client::Client, handler::MessageHandler, server::Server},
    protos::messages::RaftHeartbeat,
    DynamicClient,
};
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct StubClient {}
impl StubClient {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Client for StubClient {
    fn get_id(&self) -> i32 {
        3
    }
    fn get_server(&self) -> &Option<Arc<Mutex<Server>>> {
        &None
    }
    async fn send_bytes(&self, _buf: &[u8]) -> Result<(), Box<dyn Error>> {
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
    ) -> Result<(), Box<dyn Error>> {
        let message = msg.transcode_to::<RaftHeartbeat>().unwrap();
        assert_eq!(message.term, 4);
        Ok(())
    }
}
