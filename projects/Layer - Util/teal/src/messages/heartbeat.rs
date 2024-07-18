use crate::{net::message::MessageIdentifiable, protos::messages::{Heartbeat, RaftHeartbeat}};

impl MessageIdentifiable for Heartbeat {
    fn id(&self) -> u16 {
        3
    }
}

impl MessageIdentifiable for RaftHeartbeat {
    fn id(&self) -> u16 {
        4
    }
}
