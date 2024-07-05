#![warn(rust_2018_idioms)]
mod handlers;

use handlers::ping_handler::PingHandler;
use teal::net::handlers::MessageHandlers;
use teal::net::server::Server;
use teal::protos::gen::ping::Ping;
use teal::protos::gen::raft::Heartbeat;
use teal::raft::heartbeat_handler::HeartbeatHandler;
use std::sync::Arc;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7000".to_string());

    // Handlers
    let reg = create_handlers();

    // Server start
    Server::run(addr, Arc::new(reg)).await.ok();
    
    Ok(())
}

fn create_handlers() -> MessageHandlers{
    let mut reg = MessageHandlers::new();
    reg.register(Ping::new(), Arc::new(PingHandler));
    reg.register(Heartbeat::new(), Arc::new(HeartbeatHandler));
    return reg;
}
