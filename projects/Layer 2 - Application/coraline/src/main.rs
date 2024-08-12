pub mod api;
pub mod handlers;

use coral_commons::protos::{messages::{QueueType, SetQueueResponse}, models::Match};
use handlers::{
    created_lobby_handler::CreatedLobbyHandler, match_handler::MatchHandler,
    ping_handler::PingHandler, set_queue_response_handler::SetQueueResponseHandler,
};
use once_cell::sync::Lazy;
use realm_commons::{
    protos::{
        client::{CreateLobby, Identify},
        models::{player::PlayerState, Player},
        server::CreatedLobby,
    },
    red::{red_lobby, red_player},
};
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
    pub player: Player,
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
    let envi = env::args().nth(1).unwrap_or(".env.dev".to_string());
    dotenv::from_filename(envi).ok();

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // localhost:8000 for connections.
    let coral_url = env::var("CORAL_URL").unwrap_or("localhost:8000".to_string());
    let redis_url = env::var("REDIS_URL").unwrap_or("localhost:6379".to_string());
    
    println!("Connecting to Coral on {} and Redis on {}", coral_url, redis_url);

    let redis_client = redis::Client::open("redis://".to_string() + &redis_url)?;
    let conn = redis_client.get_connection()?;

    // Handlers
    let reg = create_handlers();

    // Client
    let client: DefaultClient =
        DefaultClient::new_connection(&coral_url, Arc::new(reg)).await?;

    let mut coraline = CORALINE.lock().await;
    coraline.client = Some(Arc::new(client));
    coraline.db = Some(conn);
    drop(coraline);

    // Start
    let _ = tokio::join!(
        // game client
        coraline_launch(),
        // api server
        api::rocket_launch()
    );
    coraline_dispose().await?;
    Ok(())
}

/**
 * Start the coraline client, create a player, and identify self on the server
 */
async fn coraline_launch() -> Result<(), JoinError> {
    println!("Coraline launch");
    let mut coraline = CORALINE.lock().await;

    // Start client
    let client_ref = coraline.client.clone().unwrap();
    let client_ref_thread = client_ref.clone();
    let t1 = tokio::spawn(async move {
        client_ref_thread.run().await.unwrap();
    });
    println!("Coraline started client");

    // Create Player
    let mut id_generator_generator = SnowflakeIdGenerator::new(1, 1);
    let player_id = id_generator_generator.real_time_generate();
    let mut player: Player = Player::default();
    if let Some(db) = &mut coraline.db {
        player.id = player_id.to_string();
        player.mmr = 1000;
        player.state = PlayerState::InLobby as i32;
        let _ = red_player::set(db, &player);
    }
    coraline.player = player;

    // Release the mutex
    drop(coraline);

    // Identify player on the server
    let identify = Identify {
        player_id: player_id.to_string(),
    };
    let identify_buf = message::serialize(&identify);
    client_ref.send_bytes(&identify_buf).await.unwrap();

    // Send a message to create a Lobby.
    // When it is created, we'll respond by setting the queue active.
    let create_lobby = CreateLobby { queue: QueueType::Idle as i32 };
    let create_lobby_buf = message::serialize(&create_lobby);
    client_ref.send_bytes(&create_lobby_buf).await.unwrap();

    println!("Coraline t1 await");
    t1.await
}

/**
 * Delete the player and lobby when we're done
 */
async fn coraline_dispose() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut coraline = CORALINE.lock().await;
    let player = coraline.player.clone();
    if let Some(db) = &mut coraline.db {
        //crate::DB {
        println!("Delete player {}", player.id);
        let lobby_id = red_player::get_lobby_by_id(db, &player.id)?;
        let _ = red_player::delete_by_id(db, &player.id)?;
        println!("Delete lobby {}", lobby_id);
        let _ = red_lobby::delete_by_id(db, &lobby_id)?;
    }
    Ok(())
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
