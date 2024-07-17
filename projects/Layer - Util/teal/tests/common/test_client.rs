use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use teal::{
    net::{
        client::{Client, DefaultClient},
        handler::{MessageHandler, MockMessageHandler},
        handlers::MessageHandlers,
        message,
    }, protos::messages::{Heartbeat, Ping}, ArcClient, BoxMessageDyn
};

pub async fn create_client() -> Result<(), Box<dyn Error>> {
    let mut mock_handler = MockMessageHandler::new();
    mock_handler.expect_handle()
        .times(1);

    // Handlers
    let mut reg = MessageHandlers::new();
    reg.register(Ping::new(), Arc::new(mock_handler));

    // Client
    let client: DefaultClient =
        DefaultClient::new_connection("127.0.0.1:8000", Arc::new(reg)).await?;
    let client_ref = Arc::new(client);
    let client_ref2 = client_ref.clone();

    // Start
    let t1 = tokio::spawn(async move {
        client_ref.run().await.unwrap();
    });

    let mut hb = Heartbeat::new();
    hb.set_leader_id(3);
    let buf = message::serialize(&hb);
    // TODO send
    client_ref2.send_bytes(&buf).await.unwrap();
    client_ref2
        .send_bytes(&message::serialize(&Ping::new()))
        .await
        .unwrap();
    // client_ref2.send(hb).await.unwrap();
    // client_ref2.send(Ping::new()).await.unwrap();
    t1.await?;

    Ok(())
}

// struct PingHandler {}
// #[async_trait]
// impl MessageHandler for PingHandler {
//     async fn handle(&self, msg: BoxMessageDyn, client: &ArcClient) -> Result<(), Box<dyn Error>> {
//         let message = msg.downcast_ref::<Ping>().unwrap();
//         println!("hey client got ping {:?}", message);
// 		Ok(())
//     }
// }
