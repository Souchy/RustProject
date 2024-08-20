use std::{error::Error, sync::Arc};

use teal::{
    net::{
        client::{Client, DefaultClient}, handler::MockMessageHandler, handlers::MessageHandlers, message
    },
    protos::messages::{Ping, RaftHeartbeat},
};

pub async fn create_client() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut mock_handler = MockMessageHandler::new();
    mock_handler.expect_handle().times(1);

    // Handlers
    let mut reg = MessageHandlers::new();
    reg.register_pool(0, Arc::new(teal::DESCRIPTOR_POOL.to_owned()));
    reg.register(0, &Ping::default(), Box::new(mock_handler));

    // Client
    let client: DefaultClient =
        DefaultClient::new_connection("127.0.0.1:8000", Arc::new(reg)).await?;
    let client_ref = Arc::new(client);
    let client_ref2 = client_ref.clone();

    // Start
    let t1 = tokio::spawn(async move {
        client_ref.run().await.unwrap();
    });

    let mut hb = RaftHeartbeat::default();
    hb.leader = 3;
    let hbbuf = message::serialize(&hb);
    client_ref2.send_bytes(&hbbuf).await.unwrap();

    let pingbuf = message::serialize(&Ping::default());
    client_ref2.send_bytes(&pingbuf).await.unwrap();
    t1.await?;

    Ok(())
}

