use std::sync::Arc;

use net::client::Client;
use net::handler::MessageHandler;
use protobuf::MessageDyn;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;

pub mod net;
pub mod protos;
pub mod messages;
pub mod raft;

pub type Reader = Arc<Mutex<OwnedReadHalf>>;
pub type Writer = Arc<Mutex<OwnedWriteHalf>>;

pub type BoxMessageDyn = Box<dyn MessageDyn>;// + Send + Sync
pub type ArcMessageHandler = Arc<dyn MessageHandler + Send + Sync>;

pub type ArcClient = (dyn Client + Send + Sync); //Box<dyn Client + Send + Sync>;
// pub type ArcClient = Arc<dyn Client + Send + Sync>;

pub const ID_LEN: usize = 2;
pub const LEN_LEN: usize = 8;
pub const HEADER_LEN: usize = ID_LEN + LEN_LEN;
