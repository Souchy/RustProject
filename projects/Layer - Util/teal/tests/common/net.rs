use std::error::Error;

use async_trait::async_trait;
use teal::{net::{client::Client, handler::MessageHandler}, protos::gen::raft::Heartbeat, ArcClient, BoxMessageDyn};


#[derive(Clone, Default)]
pub struct StubClient {}
impl StubClient {
	pub fn new() -> Self {
		Self {}
	}
}

#[async_trait]
impl Client for StubClient {
	fn get_id(&self) -> i32 {
		3
	}
	async fn send_bytes(&self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
	async fn run(&self) -> Result<(), Box<dyn Error + Send>> {
		Ok(())
	}
	async fn frame(&self, buf: &[u8]) {}
	// async fn send<T: MessageIdentifiable + MessageFull>(
	//     &self,
	//     msg: T,
	// ) -> Result<(), Box<dyn Error>> {
	//     Ok(())
	// }
	// async fn broadcast<T: MessageIdentifiable + MessageFull>(&mut self, msg: T) {}
}

pub struct HeartbeatHandlerAssertTerm4;

#[async_trait]
impl MessageHandler for HeartbeatHandlerAssertTerm4 {
	async fn handle(
		&self,
		msg: BoxMessageDyn,
		client: &ArcClient,
	) -> Result<(), Box<dyn Error>> {
		let message = msg.downcast_ref::<Heartbeat>().unwrap();
		assert_eq!(message.term(), 4);
		Ok(())
	}
}
