use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use coral_commons::protos::messages::{SetQueueRequest, SetQueueResponse};
use prost_reflect::DynamicMessage;
use realm_commons::{protos::models::player::PlayerState, red::{red_lobby, red_player}};
use teal::{
    net::{handler::MessageHandler, message::serialize},
    DynamicClient,
};

#[derive(Debug, Default)]
pub(crate) struct SetQueueHandler;
#[async_trait]
impl MessageHandler for SetQueueHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error>> {
        let message = msg.transcode_to::<SetQueueRequest>().unwrap();
        println!("hey coral got {:?}", message);

        let player_id = client.get_id().lock().await.clone();
        
        unsafe {
            if let Some(db) = &mut crate::DB {
                let mut player = red_player::get(db, &player_id)?;
                let mut lobby = red_lobby::get(db, &player.lobby)?;
                
                // println!("With player {:?}", player);
                // println!("With lobby {:?}", lobby);

                if lobby.players.contains(&player_id) {
                    let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
                    lobby.queue_start_time = time.as_secs();
                    lobby.queue = message.queue;
                    lobby.state = message.state;
                    red_lobby::set(db, &lobby)?;

                    player.state = PlayerState::InQueue as i32;
                    red_player::set_state(db, &player)?;
                    // TODO: start a thread to find a match
                }

                let response = SetQueueResponse {
                    queue: lobby.queue,
                    state: lobby.state,
                };
                let buf = serialize(&response);
                return client.send_bytes(&buf).await;
            }
        }

        Ok(())
    }
}
