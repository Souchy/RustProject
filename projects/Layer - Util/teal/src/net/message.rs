use protobuf::MessageFull;
use std::any::type_name;

use crate::{HEADER_LEN, ID_LEN, LEN_LEN};

/// v associates an Id with a proto Message
pub trait MessageIdentifiable {
    fn id(&self) -> u16;
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
}

pub fn serialize<T: MessageIdentifiable + MessageFull>(msg: &T) -> Vec<u8> {
    let mut buf = msg.write_to_bytes().unwrap();
    let length = (buf.len() + HEADER_LEN).to_be_bytes();
    let id = msg.id().to_be_bytes();
    for i in 0..ID_LEN {
        buf.insert(i, id[i]);
    }
    for i in 0..LEN_LEN {
        buf.insert(i, length[i]);
    }
    return buf;
}
