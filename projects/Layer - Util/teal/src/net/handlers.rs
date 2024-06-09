
#[async_trait]
pub trait MessageHandler {
	
    fn id(&self) -> u8;
	
    // async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>>;
}
