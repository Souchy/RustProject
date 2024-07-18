use crate::DynClient;
use async_trait::async_trait;
use core::fmt::Debug;
use mockall::predicate::*;
use mockall::*;
use prost_reflect::DynamicMessage;
use std::error::Error;

#[automock]
#[async_trait]
pub trait MessageHandler: Debug + Send + Sync {
    async fn handle(&self, msg: DynamicMessage, client: &DynClient) -> Result<(), Box<dyn Error>>;
}
