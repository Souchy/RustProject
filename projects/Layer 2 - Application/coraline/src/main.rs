pub mod api;
pub mod handlers;

use coral_commons::protos::{messages::SetQueueResponse, models::Match};
use handlers::{
    created_lobby_handler::CreatedLobbyHandler, match_handler::MatchHandler,
    ping_handler::PingHandler, set_queue_response_handler::SetQueueResponseHandler,
};
use once_cell::sync::Lazy;
use realm_commons::protos::{client::Identify, server::CreatedLobby};
use snowflake::SnowflakeIdGenerator;
use std::{env, error::Error, sync::Arc};
use teal::{
    net::{
        client::{Client, DefaultClient},
        handlers::MessageHandlers,
        message,
    },
    protos::messages::Ping,
};
use tokio::{sync::Mutex, task::JoinError};

#[derive(Default)]
pub struct Coraline {
    pub client: Option<Arc<dyn Client>>,
    // pub player: Player,
    pub player_id: String,
    // Client wouldn't have a DB connection in real life. Used as a dev shortcut.
    pub db: Option<redis::Connection>,
}
pub static CORALINE: Lazy<Mutex<Coraline>> = Lazy::new(|| Mutex::new(Coraline::default()));

/**
 * Coraline is a game client.
 * It creates a player and a game lobby to find a match on the server.
 */
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let envi = env::var("ENV_FILE").unwrap_or(".env.dev".to_string());
    let player_id = env::args().nth(1).unwrap_or_else(|| {
        let mut id_generator_generator = SnowflakeIdGenerator::new(1, 1);
        return id_generator_generator.real_time_generate().to_string();
    });
    dotenv::from_filename(envi).ok();

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // localhost:8000 for connections.
    let coral_url = env::var("CORAL_URL").unwrap();
    let redis_url = env::var("REDIS_URL").unwrap();

    println!(
        "Connecting to Coral on {} and Redis on {}",
        coral_url, redis_url
    );

    let redis_client = redis::Client::open("redis://".to_string() + &redis_url)?;
    let conn = redis_client.get_connection()?;

    // Handlers
    let reg = create_handlers();

    // Client
    let client: DefaultClient = DefaultClient::new_connection(&coral_url, Arc::new(reg)).await?;

    let mut coraline = CORALINE.lock().await;
    coraline.client = Some(Arc::new(client));
    coraline.db = Some(conn);
    coraline.player_id = player_id;
    drop(coraline);

    // Start
    let _ = tokio::join!(
        // game client
        coraline_launch(),
        // api server
        api::rocket_launch()
    );
    // coraline_dispose().await?;
    Ok(())
}

/**
 * Start the coraline client, create a player, and identify self on the server
 */
async fn coraline_launch() -> Result<(), JoinError> {
    println!("Coraline launch");
    let coraline = CORALINE.lock().await;
    let player_id = coraline.player_id.clone();

    // Start client
    let client_ref = coraline.client.clone().unwrap();
    let client_ref_thread = client_ref.clone();
    let t1 = tokio::spawn(async move {
        client_ref_thread.run().await.unwrap();
    });
    println!("Coraline started client");

    // Drop the mutex
    drop(coraline);

    // Identify player on the server
    let identify = Identify {
        player_id: player_id.clone(),
    };
    let identify_buf = message::serialize(&identify);
    client_ref.send_bytes(&identify_buf).await.unwrap();

    println!("Coraline t1 await");
    t1.await
}

/**
 * Create message handlers
 */
fn create_handlers() -> MessageHandlers {
    let mut reg = MessageHandlers::new();
    teal::register_pool(&mut reg);
    coral_commons::register_pool(&mut reg);
    realm_commons::register_pool(&mut reg);

    // teal
    reg.register(teal::POOL_ID, &Ping::default(), Box::new(PingHandler));

    // realm
    reg.register(
        realm_commons::POOL_ID,
        &CreatedLobby::default(),
        Box::new(CreatedLobbyHandler),
    );

    // coral
    reg.register(
        coral_commons::POOL_ID,
        &Match::default(),
        Box::new(MatchHandler),
    );
    reg.register(
        coral_commons::POOL_ID,
        &SetQueueResponse::default(),
        Box::new(SetQueueResponseHandler),
    );

    return reg;
}
