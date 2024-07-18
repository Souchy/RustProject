use std::{env, error::Error, sync::Arc};

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use teal::{
    net::{handler::MessageHandler, handlers::MessageHandlers, message, server::Server},
    protos::messages::Ping,
    DynClient
};

pub async fn create_server() -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8000".to_string());

    // Handlers
    let mut reg = MessageHandlers::new();
    reg.register(&Ping::default(), Box::new(PingHandler::default()));

    // Server start
    Server::run(addr, Arc::new(reg)).await.ok();

    Ok(())
}

#[derive(Debug, Default)]
struct PingHandler {}
#[async_trait]
impl MessageHandler for PingHandler {
    async fn handle(&self, msg: DynamicMessage, client: &DynClient) -> Result<(), Box<dyn Error>> {
        let message = msg.transcode_to::<Ping>();
        println!("hey server got ping {:?}", message);
        let buf = message::serialize(&Ping::default());
        return client.send_bytes(&buf).await;
    }
}
