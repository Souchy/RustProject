use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;
use std::sync::Arc;
use teal::net::handlers::MessageHandlers;

pub mod message_ids;
pub mod protos;
pub mod red;

pub static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    DescriptorPool::decode(
        include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
    )
    .unwrap()
});

pub const POOL_ID: u16 = 1;
pub fn register_pool(reg: &mut MessageHandlers) {
    reg.register_pool(POOL_ID, Arc::new(DESCRIPTOR_POOL.to_owned()));
}
