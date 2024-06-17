use crate::{ArcClient, BoxMessageDyn};
use async_trait::async_trait;
use std::error::Error;
use mockall::*;
use mockall::predicate::*;

// #[cfg(test)]
// use mockall::{automock, mock, predicate::*};
// #[cfg_attr(test, automock)]
#[automock]
#[async_trait]
pub trait MessageHandler {
    async fn handle(&self, msg: BoxMessageDyn, client: &ArcClient) -> Result<(), Box<dyn Error>>;
}
