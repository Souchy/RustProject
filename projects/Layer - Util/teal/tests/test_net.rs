mod common;

use common::net::{RaftHeartbeatHandlerAssertTerm4, StubClient};

use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use teal::{
    net::{
        handlers::MessageHandlers,
        message::{serialize, MessageIdentifiable},
    },
    protos::messages::RaftHeartbeat,
};

fn add(msg: &Arc<Mutex<dyn MessageIdentifiable>>) {
    let mut a = msg.lock().unwrap();
    let req: &mut RaftHeartbeat = a.downcast_mut::<RaftHeartbeat>().unwrap();
    req.term += 3;
}

#[test]
fn test_message_manipulation() {
    let mut req: RaftHeartbeat = RaftHeartbeat::default();
    req.term = 1;

    let msg: Arc<Mutex<dyn MessageIdentifiable>> = Arc::new(Mutex::new(req));
    add(&msg);

    let mut a = msg.lock().unwrap();
    let req: &mut RaftHeartbeat = a.downcast_mut::<RaftHeartbeat>().unwrap();

    assert_eq!(req.term, 4);
}

#[test]
fn test_message_serde() -> Result<(), Box<dyn Error>> {
    // Setup
    let mut reg = MessageHandlers::new();
    reg.register_pool(0, Arc::new(teal::DESCRIPTOR_POOL.to_owned()));
    reg.register(
        0,
        &RaftHeartbeat::default(),
        Box::new(RaftHeartbeatHandlerAssertTerm4),
    );

    // Message
    let mut req = Box::new(RaftHeartbeat::default());
    req.term = 4;

    // Generic serialization with id and length
    let buf = serialize(req.as_ref());

    // Generic deserialization from id
    let (_, dynamic_message) = reg.deserialize(&buf)?;
    let req_out = dynamic_message.transcode_to::<RaftHeartbeat>()?;

    // Test
    assert_eq!(req_out.term, 4);
    Ok(())
}

#[tokio::test]
async fn test_handlers() {
    let mut reg = MessageHandlers::new();
    reg.register_pool(0, Arc::new(teal::DESCRIPTOR_POOL.to_owned()));
    reg.register(
        0,
        &RaftHeartbeat::default(),
        Box::new(RaftHeartbeatHandlerAssertTerm4),
    );

    // Message
    let mut req = Box::new(RaftHeartbeat::default());
    req.term = 4;

    // Generic serialization with id and length
    let buf = serialize(req.as_ref());

    // Generic deserialization + Handle
    let client = StubClient::new();
    _ = reg.handle(&buf, &client).await; // RaftHeartbeatHandlerAssertTerm4 will assert that the term is 4
}

// TODO test server-client
// #[tokio::test]
// async fn test_communication() {
//     create_server().await;
//     create_client().await;
// }
