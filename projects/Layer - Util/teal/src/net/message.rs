use protobuf::MessageFull;
use std::
    any::type_name
;

/// MessageSerde associates an Id with a Message
pub trait MessageIdentifiable {
    fn id(&self) -> u8;
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
}

pub fn serialize<T: MessageIdentifiable + MessageFull>(msg: &T) -> Vec<u8> {
    let mut buf = msg.write_to_bytes().unwrap();
    buf.insert(0, (buf.len() as u8) + 2);
    buf.insert(1, msg.id());
    return buf;
}
