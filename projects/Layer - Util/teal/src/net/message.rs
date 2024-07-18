use prost::Message;
use prost_reflect::ReflectMessage;
use std::any::{type_name, Any, TypeId};
use std::fmt::Debug;

use crate::{HEADER_LEN, ID_LEN, LEN_LEN};

///Associates an Id with a proto Message
pub trait MessageIdentifiable:
    Any + Debug + Send + Sync + Message + ReflectMessage + 'static
{
    fn id(&self) -> u16;
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
}

pub fn serialize<T: MessageIdentifiable>(msg: &T) -> Vec<u8> {
    let mut buf = msg.encode_to_vec();
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

/**
 * Downcast functions sourced from rust-protobuf
 */
impl dyn MessageIdentifiable {
    /// Downcast `Box<dyn Message>` to specific message type.
    ///
    /// ```
    /// # use teal::net::message::MessageIdentifiable;
    /// # fn foo<MyMessage: MessageIdentifiable>(message: Box<dyn MessageIdentifiable>) {
    /// let m: Box<dyn MessageIdentifiable> = message;
    /// let m: Box<MyMessage> = <dyn MessageIdentifiable>::downcast_box(m).unwrap();
    /// # }
    /// ```
    pub fn downcast_box<T: Any>(
        self: Box<dyn MessageIdentifiable>,
    ) -> std::result::Result<Box<T>, Box<dyn MessageIdentifiable>> {
        if Any::type_id(&*self) == TypeId::of::<T>() {
            unsafe {
                let raw: *mut dyn MessageIdentifiable = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }

    /// Downcast `&dyn Message` to specific message type.
    ///
    /// ```
    /// # use teal::net::message::MessageIdentifiable;
    /// # fn foo<MyMessage: MessageIdentifiable>(message: &dyn MessageIdentifiable) {
    /// let m: &dyn MessageIdentifiable = message;
    /// let m: &MyMessage = <dyn MessageIdentifiable>::downcast_ref(m).unwrap();
    /// # }
    /// ```
    pub fn downcast_ref<'a, M: MessageIdentifiable + 'a>(&'a self) -> Option<&'a M> {
        if Any::type_id(&*self) == TypeId::of::<M>() {
            unsafe { Some(&*(self as *const dyn MessageIdentifiable as *const M)) }
        } else {
            None
        }
    }

    /// Downcast `&mut dyn Message` to specific message type.
    ///
    /// ```
    /// # use teal::net::message::MessageIdentifiable;
    /// # fn foo<MyMessage: MessageIdentifiable>(message: &mut dyn MessageIdentifiable) {
    /// let m: &mut dyn MessageIdentifiable = message;
    /// let m: &mut MyMessage = <dyn MessageIdentifiable>::downcast_mut(m).unwrap();
    /// # }
    /// ```
    pub fn downcast_mut<'a, M: MessageIdentifiable + 'a>(&'a mut self) -> Option<&'a mut M> {
        if Any::type_id(&*self) == TypeId::of::<M>() {
            unsafe { Some(&mut *(self as *mut dyn MessageIdentifiable as *mut M)) }
        } else {
            None
        }
    }
}
