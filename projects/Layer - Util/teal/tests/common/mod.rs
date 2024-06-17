use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use teal::{
    net::{
        client::{Client, DefaultClient},
        handler::MessageHandler,
        handlers::MessageHandlers,
        message,
    },
    protos::gen::{ping::Ping, raft::Heartbeat},
    ArcClient, BoxMessageDyn,
};

pub mod net;
pub mod test_client;
pub mod test_server;

pub fn setup() {
    // setup code specific to your library's tests would go here
}
