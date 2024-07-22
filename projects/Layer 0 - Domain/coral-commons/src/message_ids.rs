// use crate::protos::gen::{Match::Match, RequestMatch::RequestMatch, SetInQueue::SetInQueue};
use teal::net::message::MessageIdentifiable;

use crate::protos::{messages::{RequestMatch, SetInQueueRequest, SetInQueueResponse}, models::Match};

const CORAL: u16 = 8000;
const MODEL: u16 = CORAL + 0;
const REQUEST: u16 = CORAL + 100;
const RESPONSE: u16 = CORAL + 200;

impl MessageIdentifiable for Match {
    fn id(&self) -> u16 {
        MODEL + 1
    }
}

impl MessageIdentifiable for RequestMatch {
    fn id(&self) -> u16 {
        REQUEST + 1
    }
}

impl MessageIdentifiable for SetInQueueRequest {
    fn id(&self) -> u16 {
        REQUEST + 2
    }
}

impl MessageIdentifiable for SetInQueueResponse {
    fn id(&self) -> u16 {
        RESPONSE + 2
    }
}
