mod handlers;
mod api;

use coral_commons::protos::messages::SetQueueRequest;
use handlers::create_lobby_handler::CreateLobbyHandler;
use handlers::identify_handler::IdentifyHandler;
use handlers::ping_handler::PingHandler;
use handlers::raftheartbeat_handler::RaftHeartbeatHandler;
use handlers::set_queue_handler::SetQueueHandler;
use realm_commons::protos::client::{CreateLobby, Identify};
use std::env;
use std::error::Error;
use std::sync::Arc;
use teal::net::handlers::MessageHandlers;
use teal::net::server::Server;
use teal::protos::messages::{Ping, RaftHeartbeat};

pub static mut DB: Option<redis::Connection> = None;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let envi = env::args().nth(1).unwrap_or(".env.dev".to_string());
    dotenv::from_filename(envi).ok();

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // localhost:8000 for connections.
    let coral_url = env::var("CORAL_URL").unwrap_or("localhost:8000".to_string());
    let redis_url = env::var("REDIS_URL").unwrap_or("localhost:6379".to_string());

    println!("Starting Coral on {} and connecting to Redis on {}", coral_url, redis_url);

    let redis_client = redis::Client::open("redis://".to_string() + &redis_url)?;
    let conn = redis_client.get_connection()?;
    unsafe {
        DB = Some(conn);
    }

    // Handlers
    let reg = create_handlers();

    // Server start
    
    let _ = tokio::join!(
        // game client
        Server::run(coral_url, Arc::new(reg)),
        // api server
        api::rocket_launch()
    );

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
    reg.register(
        coral_commons::POOL_ID,
        &SetQueueRequest::default(),
        Box::new(SetQueueHandler),
    );

    // realm
    reg.register(
        realm_commons::POOL_ID,
        &Identify::default(),
        Box::new(IdentifyHandler),
    );
    reg.register(
        realm_commons::POOL_ID,
        &CreateLobby::default(),
        Box::new(CreateLobbyHandler),
    );

    return reg;
}
