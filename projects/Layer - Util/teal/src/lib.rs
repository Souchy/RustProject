use std::sync::Arc;
use net::client::Client;
use net::message::MessageIdentifiable;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use prost_reflect::DescriptorPool;
use once_cell::sync::Lazy;

pub mod net;
pub mod messages;
pub mod protos;

pub type Reader = Arc<Mutex<OwnedReadHalf>>;
pub type Writer = Arc<Mutex<OwnedWriteHalf>>;

pub type BoxMessageDyn = Box<dyn MessageIdentifiable>;// + Send + Sync
pub type DynClient = (dyn Client);

pub const ID_LEN: usize = 2;
pub const LEN_LEN: usize = 8;
pub const HEADER_LEN: usize = ID_LEN + LEN_LEN;

pub static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    DescriptorPool::decode(
        include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
    )
    .unwrap()
});


#[derive(thiserror::Error, Debug)]
pub enum Errors {
    #[error("The data for key '{0}' in '{1}' is not available")]
    Missing(String, String),
    #[error("Duplicate: '{0}'")]
    Duplicate(String),
}
