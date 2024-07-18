use prost_reflect::DescriptorPool;
use once_cell::sync::Lazy;

pub mod messages;
pub mod protos;

pub static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    DescriptorPool::decode(include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
    )
    .unwrap()
});
