use async_trait::async_trait;
use coral_commons::protos::messages::{QueueType, SetQueueRequest, SetQueueResponse};
use coral_commons::protos::models::{Match, MatchState};
use coral_commons::red::red_match;
use once_cell::sync::Lazy;
use prost_reflect::DynamicMessage;
use realm_commons::{
    protos::models::{player::PlayerState, Lobby, LobbyState},
    red::{red_lobby, red_player},
};
use snowflake::SnowflakeIdGenerator;
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
        
        println!("With player_id {:?}", player_id);

        unsafe {
            if let Some(db) = &mut crate::DB {
                let mut player = red_player::get(db, &player_id)?;
                println!("With player {:?}", player);
                let mut lobby = red_lobby::get(db, &player.lobby)?;
                println!("With lobby {:?}", lobby);

                if lobby.players.contains(&player_id) {
                    let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
                    let mut total_mmr: u32 = 0;
                    for play in &lobby.players {
                        let mmr = red_player::get_mmr_by_id(db, &play)?;
                        println!("Add player {} mmr {} to average ", play, mmr);
                        total_mmr += mmr;
                    }

                    // Remove from the queue index always before updating the queue value
                    red_lobby::delete_mmr_index(db, &lobby.id, &lobby.queue)?;

                    // If there is already a queue thread for this lobby, terminate it
                    if let Some(task) = QUEUES.lock().await.tasks.remove(&lobby.id) {
                        task.abort();
                        println!("Cancelled queue for lobby {}", lobby.id);
                    }

                    //Update
                    lobby.queue = message.queue;
                    lobby.queue_start_time = time.as_secs();
                    lobby.average_mmr = total_mmr / lobby.players.len() as u32;
                    println!("Average mmr: {}", lobby.average_mmr);

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
    lobby1: Lobby,
    server: Arc<Mutex<Server>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // let lobby2: Lobby = Lobby::default();
    let mut i = 0;

    unsafe {
        if let Some(db) = &mut crate::DB {
            loop {
                println!("In task! {}", i);
                i = i + 1;
                tokio::time::sleep(Duration::from_secs(2)).await;

                // let ids = red_lobby::get_ids(db);
                let result = red_lobby::find_lobby_match(db, &lobby1);
                if result.is_err() {
                    continue;
                }
                if let Ok(None) = result {
                    continue;
                }
                let lobby2id = result.unwrap().unwrap();
                let lobby2res = red_lobby::get(db, &lobby2id);
                if lobby2res.is_err() {
                    continue;
                }
                let lobby2 = lobby2res.unwrap();

                println!("Found match {} + {}", lobby1.id, lobby2.id);

                // Lock Queues
                let mut queues = QUEUES.lock().await;
                // If task was terminated by an other thread
                if !queues.tasks.contains_key(&lobby1.id) {
                    println!("Queue thread got terminated by another one");
                    return Ok(());
                }
                queues.tasks.remove(&lobby1.id);
                let t2 = queues.tasks.remove(&lobby2.id);
                if let Some(task) = t2 {
                    task.abort();
                }
                drop(queues);

                // Create Match

                let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
                let mut id_generator_generator = SnowflakeIdGenerator::new(1, 1);
                let game_id = id_generator_generator.real_time_generate();
                let mut game = Match {
                    id: game_id.to_string(),
                    date: time.as_secs(),
                    state: MatchState::Active as i32,
                    queue: lobby1.queue,
                    game_port: 9999,
                    token: "".to_string(),
                    players: HashMap::new(),
                };
                lobby1.players.iter().for_each(|p| {
                    game.players.insert(p.clone(), lobby1.id.clone());
                });
                lobby2.players.iter().for_each(|p| {
                    game.players.insert(p.clone(), lobby2.id.clone());
                });
                red_match::set(db, &game).ok();

                // Delete the lobbies and the queue threads
                let _ = red_lobby::delete(db, &lobby1);
                let _ = red_lobby::delete(db, &lobby2);

                let serv = server.lock().await;
                let clients = &serv.clients;
                let match_buf = serialize(&game);

                // For all players in the game
                for (id, team_id) in &game.players {
                    // Set players in game
                    let _ = red_player::set_state_by_id(db, id, PlayerState::InGame);
                    let _ = red_player::set_game_by_id(db, id, &game.id);
                    let _ = red_player::set_lobby_by_id(db, id, &"".to_string());

                    // Find Client corresponding to the player
                    for c in clients {
                        let re = c.get_id_ref();
                        let id2 = re.lock().await;
                        if id2.eq(id) {
                            // Send match
                            println!("Send match to client {}", id);
                            let _ = c.send_bytes(&match_buf).await;
                        }
                    }
                }

                println!("Queue thread complete");
                return Ok(());
            }
        }
    }

    Ok(())
}

// pub async fn compare_id(client: Arc<dyn Client>, id: &String) -> bool { // Result<bool, Box<dyn Error + Send + Sync>> {
//     client.get_id_ref().lock().await.eq(id)
// }
