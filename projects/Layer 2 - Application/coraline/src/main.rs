pub mod handlers;

use std::{error::Error, sync::Arc};

use handlers::ping_handler::PingHandler;
use teal::{net::{client::{Client, DefaultClient}, handlers::MessageHandlers, message}, protos::messages::{Heartbeat, Ping, RaftHeartbeat}};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Handlers
    let reg = create_handlers();

    // Client
    let client: DefaultClient = DefaultClient::new_connection("127.0.0.1:8000", Arc::new(reg)).await?;
    let client_ref = Arc::new(client);
    let client_ref2 = client_ref.clone();

    // Start
    let t1 = tokio::spawn(async move {
        println!("t1 start");
        client_ref.run().await.unwrap();
    });

    // Send Heartbeat
    let mut hb = RaftHeartbeat::default();
    hb.leader = 3;
    let buf = message::serialize(&hb);
    client_ref2.send_bytes(&buf).await.unwrap();
    // Send Ping
    client_ref2.send_bytes(&message::serialize(&Ping::default())).await.unwrap();
    // TODO send
    // client_ref2.send(hb).await.unwrap();
    // client_ref2.send(Ping::new()).await.unwrap();
    t1.await?;

    Ok(())
}

fn create_handlers() -> MessageHandlers {
    let mut reg = MessageHandlers::new();
    teal::register_pool(&mut reg);
    coral_commons::register_pool(&mut reg);

    reg.register(teal::POOL_ID, &Ping::default(), Box::new(PingHandler));

    return reg;
}
