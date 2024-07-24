use std::error::Error;

use async_trait::async_trait;
use prost_reflect::DynamicMessage;
use realm_commons::{
    protos::{
        client::CreateLobby,
        models::{player::PlayerState, Lobby, LobbyState},
        server::CreatedLobby,
    },
    red::{red_lobby, red_player},
};
use snowflake::SnowflakeIdGenerator;
use teal::{
    net::{handler::MessageHandler, message::serialize},
    DynamicClient,
};

#[derive(Debug, Default)]
pub(crate) struct CreateLobbyHandler;
#[async_trait]
impl MessageHandler for CreateLobbyHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<CreateLobby>().unwrap();
        println!("hey coral got {:?}", message);

        let mut id_generator_generator = SnowflakeIdGenerator::new(1, 1);
        let id = id_generator_generator.real_time_generate();
        let player_id = client.get_id_ref().lock().await.clone();
        // println!("With player {}", player_id);

        // Create lobby
        let mut lobby = Lobby::default();
        lobby.id = id.to_string();
        lobby.queue = message.queue;
        lobby.state = LobbyState::Idle as i32;
        lobby.token = "token".to_string();
        lobby.players.push(player_id.clone());

        // Set lobby in Redis
        unsafe {
            if let Some(db) = &mut crate::DB {
                red_lobby::set(db, &lobby)?;

                let mut player = red_player::get(db, &player_id)?;
                player.lobby = lobby.id.clone();
                player.state = PlayerState::InLobby as i32;
                red_player::set(db, &player)?;
            }
        }

        // Return lobby info to player
        let mut response = CreatedLobby::default();
        response.lobby = lobby.id.clone();
        response.queue = 1;
        response.token = lobby.token.clone();
        let buf = serialize(&response);
        client.send_bytes(&buf).await
    }
}
