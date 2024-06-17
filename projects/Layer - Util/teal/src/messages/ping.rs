use crate::{
    net::message::MessageIdentifiable,
    protos::gen::ping::{Ping, Pong},
};

impl MessageIdentifiable for Ping {
    fn id(&self) -> u8 {
        1
    }
}

impl MessageIdentifiable for Pong {
    fn id(&self) -> u8 {
        2
    }
}
