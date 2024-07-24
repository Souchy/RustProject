use std::{
    error::Error,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use coral_commons::protos::{
    messages::{SetQueueRequest, SetQueueResponse},
    models::Match,
};
use prost_reflect::DynamicMessage;
use realm_commons::{
    protos::models::{player::PlayerState, Lobby},
    red::{red_lobby, red_player},
};
use redis::Iter;
use teal::{
    net::{client::Client, handler::MessageHandler, message::serialize, server::Server},
    DynamicClient,
};
use tokio::sync::Mutex;

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

        let server = client.get_server();
        let player_id = client.get_id_ref().lock().await.clone();

        unsafe {
            if let Some(db) = &mut crate::DB {
                let mut player = red_player::get(db, &player_id)?;
                let mut lobby = red_lobby::get(db, &player.lobby)?;

                // println!("With player {:?}", player);
                // println!("With lobby {:?}", lobby);

                if lobby.players.contains(&player_id) {
                    let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
                    let mut total_mmr: u32 = 0;
                    for play in &lobby.players {
                        let mmr = red_player::get_mmr_by_id(db, &play)?;
                        println!("Add player {} mmr {} to average ", play, mmr);
                        total_mmr += mmr;
                    }

                    lobby.state = message.state;
                    lobby.queue = message.queue;
                    lobby.queue_start_time = time.as_secs();
                    lobby.average_mmr = total_mmr / lobby.players.len() as u32;
                    println!("Average mmr: {}", lobby.average_mmr);
                    red_lobby::set(db, &lobby)?;

                    player.state = PlayerState::InQueue as i32;
                    red_player::set_state(db, &player)?;

                    // TDOO Try to find a match for the lobby
                    // let lobby2 = lobby.clone();
                    // tokio::spawn(async move {
                    //     let _result = find_match(lobby2, server).await;
                    // });
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

// TODO find a match between 2 lobbies
async fn find_match(lobby: Lobby, server: Arc<Mutex<Server>>) -> Result<(), Box<dyn Error>> {
    let lobby2: Lobby = Lobby::default();

    unsafe {
        if let Some(db) = &mut crate::DB {
            let ids = red_lobby::get_ids(db);
            let lobbies: redis::Iter<'_, String> = redis::cmd("FT.SEARCH")
                .arg("idx:lobby \"@average_mmr:[1000 +inf]\"")
                .clone()
                .iter(db)?;
        }
    }

    let r#match = Match::default();
    let match_buf = serialize(&r#match);

    let serv = server.lock().await;
    let clients = &serv.clients;
    for id in &lobby.players {
        // clients.fin
        if let Some(client) = clients.iter().find(|&c| c.get_id_sync().eq(id)) {
            let _ = client.send_bytes(&match_buf).await;
        }
    }

    Ok(())
}
