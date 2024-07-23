pub mod handlers;

use std::{error::Error, sync::Arc};

use coral_commons::protos::{messages::{QueueState, SetQueueRequest, SetQueueResponse}, models::Match};
use handlers::{created_lobby_handler::CreatedLobbyHandler, match_handler::MatchHandler, ping_handler::PingHandler, set_queue_response_handler::SetQueueResponseHandler};
use realm_commons::{
    protos::{
        client::{CreateLobby, Identify},
        models::{player::PlayerState, Player}, server::CreatedLobby,
    },
    red::red_player,
};
use redis::Commands;
use snowflake::SnowflakeIdGenerator;
use teal::{
    net::{
        client::{Client, DefaultClient},
        handlers::MessageHandlers,
        message,
    },
    protos::messages::{Ping, RaftHeartbeat},
};

pub static mut DB: Option<redis::Connection> = None;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1:6371/")?;
    let con = client.get_connection()?;
    unsafe {
        DB = Some(con);
    }

    // Temporary redis setup
    unsafe {
        if let Some(db) = &mut crate::DB {
            let _ = red_player::delete_all(db);
            // for i in 0..1000 {
            //     let mut player: Player = Player::default();
            //     player.id = i.to_string();
            //     player.mmr = 1000;
            //     player.state = PlayerState::InLobby as i32;
            //     let _ = red_player::set(db, &player);
            // }
        }
    }
    // End Temporary redis setup

    // Handlers
    let reg = create_handlers();

    // Client
    let client: DefaultClient =
        DefaultClient::new_connection("127.0.0.1:8000", Arc::new(reg)).await?;
    let client_ref = Arc::new(client);
    let client_ref2 = client_ref.clone();

    // Start
    let t1 = tokio::spawn(async move {
        println!("t1 start");
        client_ref.run().await.unwrap();
    });

    let mut id_generator_generator = SnowflakeIdGenerator::new(1, 1);
    let player_id = id_generator_generator.real_time_generate();
    
    let mut player: Player = Player::default();
    unsafe {
        if let Some(db) = &mut crate::DB {
            player.id = player_id.to_string();
            player.mmr = 1000;
            player.state = PlayerState::InLobby as i32;
            let _ = red_player::set(db, &player);
        }
    }

    let identify = Identify { player_id: player_id.to_string() };
    let identify_buf = message::serialize(&identify);
    client_ref2.send_bytes(&identify_buf).await.unwrap();

    // Send a message to create a Lobby. When it is created, we'll respond by setting the queue active.
    let create_lobby = CreateLobby { queue: 1 };
    let create_lobby_buf = message::serialize(&create_lobby);
    client_ref2.send_bytes(&create_lobby_buf).await.unwrap();

    /*
    // Send Heartbeat
    let mut hb = RaftHeartbeat::default();
    hb.leader = 3;
    let buf = message::serialize(&hb);
    client_ref2.send_bytes(&buf).await.unwrap();
    // Send Ping
    client_ref2
        .send_bytes(&message::serialize(&Ping::default()))
        .await
        .unwrap();
    // TODO send
    // client_ref2.send(hb).await.unwrap();
    // client_ref2.send(Ping::new()).await.unwrap();
    */
    t1.await?;

    // unsafe {
    //     if let Some(db) = &mut crate::DB {
    //         let _ = red_player::delete(db, &player);
    //     }
    // }
    Ok(())
}

fn create_handlers() -> MessageHandlers {
    let mut reg = MessageHandlers::new();
    teal::register_pool(&mut reg);
    coral_commons::register_pool(&mut reg);
    realm_commons::register_pool(&mut reg);

    // teal
    reg.register(teal::POOL_ID, &Ping::default(), Box::new(PingHandler));
    // realm
    reg.register(realm_commons::POOL_ID, &CreatedLobby::default(), Box::new(CreatedLobbyHandler));
    // coral
    reg.register(coral_commons::POOL_ID, &Match::default(), Box::new(MatchHandler));
    reg.register(coral_commons::POOL_ID, &SetQueueResponse::default(), Box::new(SetQueueResponseHandler));

    return reg;
}
