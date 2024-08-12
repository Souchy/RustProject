// use crate::protos::gen::{Match::Match, RequestMatch::RequestMatch, SetInQueue::SetInQueue};
use teal::net::message::MessageIdentifiable;

use crate::protos::{messages::{SetQueueRequest, SetQueueResponse}, models::{Match, MatchResult}};

const CORAL: u16 = 8000;
const MODEL: u16 = CORAL + 0;
const REQUEST: u16 = CORAL + 100;
const RESPONSE: u16 = CORAL + 200;

impl MessageIdentifiable for Match {
    fn id(&self) -> u16 {
        MODEL + 1
    }
}

impl MessageIdentifiable for MatchResult {
    fn id(&self) -> u16 {
        MODEL + 1
    }
}

impl MessageIdentifiable for SetQueueRequest {
    fn id(&self) -> u16 {
        REQUEST + 1
    }
}

impl MessageIdentifiable for SetQueueResponse {
    fn id(&self) -> u16 {
        RESPONSE + 1
    }
}
