use crate::protos::gen::messages::RequestMatch;
use teal::net::message::MessageIdentifiable;

impl MessageIdentifiable for RequestMatch {
    fn id(&self) -> u8 {
        102
    }
}
