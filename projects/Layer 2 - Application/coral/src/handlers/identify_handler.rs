use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use realm_commons::{
    protos::{
        client::Identify,
        models::{player::PlayerState, Player},
    },
    red::red_player,
};
use std::error::Error;
use teal::{net::handler::MessageHandler, DynamicClient};

#[derive(Debug, Default)]
pub(crate) struct IdentifyHandler;
#[async_trait]
impl MessageHandler for IdentifyHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<Identify>().unwrap();
        println!("hey coral got {:?}", message);

        let player_id = message.player_id;
        client.set_id(player_id.clone()).await?;

        unsafe {
            if let Some(db) = &mut crate::DB {
                let opt_player = red_player::get(db, &player_id).ok();
                if opt_player.is_none() {
                    let mut player: Player = Player::default();
                    player.id = player_id.clone();
                    player.lobby = "0".to_string();
                    player.mmr = 1000;
                    player.state = PlayerState::InLobby as i32;
                    player.recent_matches = vec![];
                    let _ = red_player::set(db, &player);
                }
            }
        }

        Ok(())
    }
}
