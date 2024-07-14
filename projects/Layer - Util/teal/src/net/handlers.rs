use crate::{
    net::message::MessageIdentifiable, ArcClient, ArcMessageHandler, BoxMessageDyn, HEADER_LEN,
    ID_LEN, LEN_LEN,
};
use protobuf::MessageFull;
use std::{collections::HashMap, error::Error};

pub struct MessageHandlers {
    deserializers: HashMap<u16, BoxMessageDyn>,
    handlers: HashMap<u16, ArcMessageHandler>,
    pub invalid_message_handler: fn(&[u8], &ArcClient),
}

impl MessageHandlers {
    pub fn new() -> Self {
        Self {
            deserializers: HashMap::new(),
            handlers: HashMap::new(),
            invalid_message_handler: Self::default_handler,
        }
    }

    pub fn register<T: MessageIdentifiable + MessageFull>(
        &mut self,
        msg: T,
        handler: ArcMessageHandler,
    ) {
        let id = msg.id();
        if self.deserializers.contains_key(&id) {
            panic!("Shouldn't register the same message deserializer + handler ID twice. Register only once or change the message ID to avoid duplicates.");
        }
        self.deserializers.insert(id, Box::new(msg));
        self.handlers.insert(id, handler);
    }

    fn deserialize(&self, frame: &[u8]) -> Option<(u16, BoxMessageDyn)> {
        let mut dst = [0u8; LEN_LEN];
        dst.clone_from_slice(&frame[0..LEN_LEN]);
        let len = usize::from_be_bytes(dst);
        let mut dstid = [0u8; ID_LEN];
        dstid.clone_from_slice(&frame[LEN_LEN..HEADER_LEN]);
        let id = u16::from_be_bytes(dstid);

        let mut msg: BoxMessageDyn = self.deserializers.get(&id)?.clone_box();
        let _ = msg.merge_from_bytes_dyn(&frame[HEADER_LEN..len]);
        return Some((id, msg));
    }

    pub async fn handle(&self, frame: &[u8], client: &ArcClient) -> Result<(), Box<dyn Error>> {
        match self.deserialize(&frame) {
            Some(tuple) => {
                if let Some(handler) = self.handlers.get(&tuple.0) {
                    return handler.handle(tuple.1, client).await;
                }
                Ok(())
            }
            None => {
                (self.invalid_message_handler)(&frame, client);
                Ok(())
            }
        }
    }

    fn default_handler(frame: &[u8], _client: &ArcClient) {
        let st = std::str::from_utf8(&frame).unwrap();
        println!("received invalid message: {}", st);
    }
}
