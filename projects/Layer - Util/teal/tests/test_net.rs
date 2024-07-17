mod common;

use common::{net::{HeartbeatHandlerAssertTerm4, StubClient}, test_client::create_client, test_server::create_server};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use protobuf::MessageDyn;
use teal::{
    net::{
        handler::MockMessageHandler, handlers::MessageHandlers, message::{serialize, MessageIdentifiable}
    },
    protos::gen::raft::Heartbeat,
    BoxMessageDyn,
};

fn add(msg: &Arc<Mutex<dyn MessageDyn>>) {
    let mut a = msg.lock().unwrap();
    let req: &mut Heartbeat = a.downcast_mut::<Heartbeat>().unwrap();
    req.set_term(req.term() + 3);
}

#[test]
fn test_message_manipulation() {
    let mut req: Heartbeat = Heartbeat::new();
    req.set_term(1);

    let msg: Arc<Mutex<dyn MessageDyn>> = Arc::new(Mutex::new(req));
    add(&msg);

    let mut a = msg.lock().unwrap();
    let req: &mut Heartbeat = a.downcast_mut::<Heartbeat>().unwrap();

    assert_eq!(req.term(), 4);
}

#[test]
fn test_message_serde() {
    // Setup
    let mut deserializers: HashMap<u16, BoxMessageDyn> = HashMap::new();
    let model_req = Heartbeat::new();
    deserializers.insert(model_req.id(), Box::new(model_req));

    // Message
    let mut req = Box::new(Heartbeat::new());
    req.set_term(4);

    // Generic serialization with id and length
    let buf = serialize(req.as_ref());

    // Generic deserialization from id
    let n: usize = buf[0] as usize;
    let id = buf[1] as u16;
    let mut ms = deserializers[&id].clone();
    let _ = ms.merge_from_bytes_dyn(&buf[2..n]);

    // handle
    let req_out: &Heartbeat = ms.downcast_ref::<Heartbeat>().unwrap();
    assert_eq!(req_out.term(), 4);
}

#[tokio::test]
async fn test_handlers() {
    let mut reg = MessageHandlers::new();
    reg.register(Heartbeat::new(), Arc::new(HeartbeatHandlerAssertTerm4));

    // Message
    let mut req = Box::new(Heartbeat::new());
    req.set_term(4);

    // Generic serialization with id and length
    let buf = serialize(req.as_ref());

    // Generic deserialization + Handle
    let client = StubClient::new();
    _ = reg.handle(&buf, &client).await;
}


// TODO test server-client
// #[tokio::test]
// async fn test_communication() {
//     create_server().await;
//     create_client().await;
// }
