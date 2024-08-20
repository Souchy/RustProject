use crate::{
    net::message::MessageIdentifiable,
    protos::messages::{Ping, Pong},
};

impl MessageIdentifiable for Ping {
    fn id(&self) -> u16 {
        1
    }
}

impl MessageIdentifiable for Pong {
    fn id(&self) -> u16 {
        2
    }
}
