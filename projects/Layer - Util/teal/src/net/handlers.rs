use prost_reflect::{DescriptorPool, DynamicMessage, MessageDescriptor};

use crate::{
    net::message::MessageIdentifiable, DynamicClient, Errors, HEADER_LEN, ID_LEN, LEN_LEN,
};
use std::{collections::HashMap, error::Error, sync::Arc};

use super::handler::MessageHandler;

#[derive(Debug)]
pub struct MessageHandlers {
    pools: HashMap<u16, Arc<DescriptorPool>>,
    deserializers: HashMap<u16, MessageDescriptor>,
    handlers: HashMap<u16, Box<dyn MessageHandler>>,
    // pub invalid_message_handler: fn(&[u8], &ArcClient),
}

impl MessageHandlers {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            deserializers: HashMap::new(),
            handlers: HashMap::new(),
            // invalid_message_handler: Self::default_handler,
        }
    }

    /**
     * Register a message library.
     */
    pub fn register_pool(&mut self, pool_id: u16, pool: Arc<DescriptorPool>) {
        if self.pools.contains_key(&pool_id) {
            panic!("Tried to register a pool twice #{}.", pool_id);
        }
        self.pools.insert(pool_id, pool);
    }

    /**
     * Register a message handler.
     */
    pub fn register(
        &mut self,
        pool_id: u16,
        msg: &dyn MessageIdentifiable,
        handler: Box<dyn MessageHandler>,
    ) {
        let id = msg.id();
        if self.deserializers.contains_key(&id) {
            panic!("Tried to register the same message deserializer + handler ID twice. Register only once or change the message ID to avoid duplicates.");
        }
        if !self.pools.contains_key(&pool_id) {
            panic!(
                "Tried to register a message to an unexisting pool #{}. Register the pools first.",
                pool_id
            );
        }
        let pool = self.pools.get(&pool_id).unwrap(); //&(id / 1000)).unwrap();
        let message_descriptor = pool
            .get_message_by_name(msg.descriptor().full_name())
            .unwrap();

        self.deserializers.insert(id, message_descriptor);
        self.handlers.insert(id, handler);
    }

    pub fn deserialize(
        &self,
        frame: &[u8],
    ) -> Result<(u16, DynamicMessage), Box<dyn Error + Send + Sync>> {
        let mut dst = [0u8; LEN_LEN];
        dst.clone_from_slice(&frame[0..LEN_LEN]);
        let len = usize::from_be_bytes(dst);
        let mut dstid = [0u8; ID_LEN];
        dstid.clone_from_slice(&frame[LEN_LEN..HEADER_LEN]);
        let id = u16::from_be_bytes(dstid);
        let bytes = &frame[HEADER_LEN..len];

        let message_descriptor = self
            .deserializers
            .get(&id)
            .ok_or(Errors::Missing(id.to_string(), "deserializers".to_string()))?
            .clone();

        let dynamic_message = DynamicMessage::decode(message_descriptor, bytes)?;
        Ok((id, dynamic_message))
    }

    pub async fn handle(
        &self,
        frame: &[u8],
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (id, dynamic_message) = self.deserialize(&frame)?;
        let handler = self
            .handlers
            .get(&id)
            .ok_or(Errors::Missing(id.to_string(), "handlers".to_string()))?;
        return handler.handle(dynamic_message, client).await;
    }

    // fn default_handler(frame: &[u8], _client: &DynamicClient) {
    //     let st = std::str::from_utf8(&frame).unwrap();
    //     println!("received invalid message: {}", st);
    // }
}
