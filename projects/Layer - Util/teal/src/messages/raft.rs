use crate::{net::message::MessageIdentifiable, protos::gen::raft::Heartbeat};


impl MessageIdentifiable for Heartbeat {
    fn id(&self) -> u8 {
        11
    }
}
