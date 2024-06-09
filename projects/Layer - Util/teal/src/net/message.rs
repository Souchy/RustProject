
#[async_trait]
pub trait MessageScript {
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
    fn id(&self) -> u8;
    // fn serialize(&self) -> Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn deserialize(bytes: &[u8]) -> Arc<dyn MessageScript + Sync + Send> {
        let i = bincode::deserialize::<Self>(&bytes[..]).unwrap();
        return Arc::new(i);
    }
    // async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>>;
    async fn send(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        let mut buf = MessageScript::serialize(self);
        buf.insert(0, buf.len() as u8 + 2);
        buf.insert(1, self.id());
        client.send_bytes(&buf).await
    }
}
