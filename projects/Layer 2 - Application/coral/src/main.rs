#![warn(rust_2018_idioms)]
mod handlers;

use handlers::ping_handler::PingHandler;
use handlers::raftheartbeat_handler::RaftHeartbeatHandler;
use teal::net::handlers::MessageHandlers;
use teal::net::server::Server;
use teal::protos::messages::{Ping, RaftHeartbeat};

use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    reg.register(teal::POOL_ID, &Ping::default(), Box::new(PingHandler));
    reg.register(
        teal::POOL_ID,
        &RaftHeartbeat::default(),
        Box::new(RaftHeartbeatHandler),
    );

    return reg;
}
