pub mod handlers;

use std::{error::Error, sync::Arc};

use handlers::ping_handler::PingHandler;
use realm_commons::{protos::models::{player::PlayerState, Player}, red::red_player};
use redis::Commands;
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
            for i in 0..1000 {
                let mut player: Player = Player::default();
                player.id = i.to_string();
                player.mmr = 1000;
                player.state = PlayerState::InLobby as i32;
                let _ = red_player::set(db, &player);
            }
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
    t1.await?;

    Ok(())
}

fn create_handlers() -> MessageHandlers {
    let mut reg = MessageHandlers::new();
    teal::register_pool(&mut reg);
    coral_commons::register_pool(&mut reg);

    reg.register(teal::POOL_ID, &Ping::default(), Box::new(PingHandler));

    return reg;
}
