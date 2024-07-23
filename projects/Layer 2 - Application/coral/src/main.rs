#![warn(rust_2018_idioms)]
mod handlers;

use coral_commons::protos::messages::SetQueueRequest;
use handlers::create_lobby_handler::CreateLobbyHandler;
use handlers::identify_handler::IdentifyHandler;
use handlers::ping_handler::PingHandler;
use handlers::raftheartbeat_handler::RaftHeartbeatHandler;
use handlers::set_queue_handler::SetQueueHandler;
use once_cell::sync::OnceCell;
use realm_commons::protos::client::{CreateLobby, Identify};
use teal::net::handlers::MessageHandlers;
use teal::net::server::Server;
use teal::protos::messages::{Ping, RaftHeartbeat};

use std::env;
use std::error::Error;
use std::sync::Arc;


pub static mut DB: Option<redis::Connection> = None;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let client = redis::Client::open("redis://127.0.0.1:6371/")?;
    let con = client.get_connection()?;
    unsafe {
        DB = Some(con);
    }

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8000".to_string());

    // Handlers
    let reg = create_handlers();

    // Server start
    Server::run(addr, Arc::new(reg)).await.ok();

    Ok(())
}

fn create_handlers() -> MessageHandlers {
    let mut reg = MessageHandlers::new();
    teal::register_pool(&mut reg);
    coral_commons::register_pool(&mut reg);
    realm_commons::register_pool(&mut reg);

    // teal
    reg.register(teal::POOL_ID, &Ping::default(), Box::new(PingHandler));
    reg.register(
        teal::POOL_ID,
        &RaftHeartbeat::default(),
        Box::new(RaftHeartbeatHandler),
    );
    // coral
    reg.register(coral_commons::POOL_ID, &SetQueueRequest::default(), Box::new(SetQueueHandler));
    // realm
    reg.register(realm_commons::POOL_ID, &Identify::default(), Box::new(IdentifyHandler));
    reg.register(realm_commons::POOL_ID, &CreateLobby::default(), Box::new(CreateLobbyHandler));

    return reg;
}
