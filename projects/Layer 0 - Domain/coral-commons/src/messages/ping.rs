
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PingMsg {}


#[async_trait]
impl MessageScript for PingMsg {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
    pub fn uid() -> u8 {
        1
    }
}
