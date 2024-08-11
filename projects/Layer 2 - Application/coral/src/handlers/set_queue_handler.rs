use async_trait::async_trait;
use coral_commons::protos::messages::{QueueType, SetQueueRequest, SetQueueResponse};
use once_cell::sync::Lazy;
use prost_reflect::DynamicMessage;
use realm_commons::{
    protos::models::{player::PlayerState, Lobby, LobbyState},
    red::{red_lobby, red_player},
};
use std::{
    collections::HashMap,
    error::Error,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use teal::{
    net::{handler::MessageHandler, message::serialize, server::Server},
    DynamicClient,
};
use tokio::{sync::Mutex, task::JoinHandle};

#[derive(Debug, Default)]
pub struct Queues {
    /**
     * All queue tasks per lobby id
     */
    tasks: HashMap<String, JoinHandle<()>>,
}
pub static QUEUES: Lazy<Mutex<Queues>> = Lazy::new(|| Mutex::new(Queues::default()));

#[derive(Debug, Default)]
pub(crate) struct SetQueueHandler;
#[async_trait]
impl MessageHandler for SetQueueHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
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

                    lobby.queue = message.queue;
                    lobby.queue_start_time = time.as_secs();
                    lobby.average_mmr = total_mmr / lobby.players.len() as u32;
                    println!("Average mmr: {}", lobby.average_mmr);

                    // If there is already a queue thread for this lobby, terminate it
                    if let Some(task) = QUEUES.lock().await.tasks.remove(&lobby.id) {
                        task.abort();
                        println!("Cancelled queue for lobby {}", lobby.id);
                    }

                    if message.queue == QueueType::Idle as i32 {
                        lobby.state = LobbyState::Idle as i32;
                        player.state = PlayerState::InLobby as i32;
                        red_player::set_state(db, &player)?;
                        red_lobby::set(db, &lobby)?;
                    } else {
                        // if message.state == QueueState::Active as i32 {
                        lobby.state = LobbyState::InQueue as i32;
                        player.state = PlayerState::InQueue as i32;
                        red_player::set_state(db, &player)?;
                        red_lobby::set(db, &lobby)?;

                        // TODO Try to find a match for the lobby
                        let lobby_find_match = lobby.clone();
                        let task = tokio::spawn(async move {
                            let _result = find_match(lobby_find_match, server).await;
                        });
                        QUEUES.lock().await.tasks.insert(lobby.id.clone(), task);
                        println!("Activated queue for lobby {}", lobby.id);
                    }
                }

                let response = SetQueueResponse { queue: lobby.queue };
                let buf = serialize(&response);
                return client.send_bytes(&buf).await;
            }
        }

        Ok(())
    }
}

// TODO find a match between 2 lobbies
async fn find_match(
    lobby: Lobby,
    server: Arc<Mutex<Server>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // let lobby2: Lobby = Lobby::default();
    let mut i = 0;
    loop {
        println!("In task! {}", i);
        i = i + 1;
        tokio::time::sleep(Duration::from_secs(3)).await;
    }

    // unsafe {
    //     if let Some(db) = &mut crate::DB {
    //         loop {
    //             // let ids = red_lobby::get_ids(db);
    //             let lobby2id = red_lobby::find_lobby_match(db, &lobby);
    //             if lobby2id.is_err() {
    //                 continue;
    //             }

    //             // TODO: query lobbies that have a different ID, have state=InQueue and have average_mmr=within { lobby.mmr -100, lobby.mmr + 100 };
    //             // let lobbies: redis::Iter<'_, String> = redis::cmd("FT.SEARCH")
    //             //     .arg("idx:lobby \"@average_mmr:[1000 +inf]\"")
    //             //     .clone()
    //             //     .iter(db)?;

    //             // TODO: compare lobbies mmr with the given lobby.

    //             // FIXME return if lobby state changes or players leave.
    //             if true {
    //                 return Ok(());
    //             }

    //             // FIXME if found a match
    //             if false {
    //                 let r#match = Match::default();
    //                 let match_buf = serialize(&r#match);

    //                 let serv = server.lock().await;
    //                 let clients = &serv.clients;
    //                 for id in &lobby.players {
    //                     // clients.fin
    //                     if let Some(client) = clients.iter().find(|&c| c.get_id_sync().eq(id)) {
    //                         let _ = client.send_bytes(&match_buf).await;
    //                     }
    //                 }
    //                 return Ok(());
    //             }
    //         }
    //     }
    // }

    Ok(())
}
