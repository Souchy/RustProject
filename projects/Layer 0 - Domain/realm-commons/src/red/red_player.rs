use std::error::Error;

use redis::Commands;

use crate::protos::models::{player::PlayerState, Player};

// Keys
fn get_key_player(player: &String) -> String {
    "player:".to_string() + player
}
const KEY_STATE: &str = "state";
const KEY_LOBBY: &str = "lobby";
const KEY_GAME: &str = "game";
const KEY_MMR: &str = "mmr";
const KEY_PLAYER_INDEX: &str = "player_ids";

// Values
pub fn set(
    db: &mut redis::Connection,
    player: &Player,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    set_lobby(db, &player)?;
    set_game(db, &player)?;
    set_mmr(db, &player)?;
    set_state(db, &player)?;
    db.sadd(KEY_PLAYER_INDEX, &player.id)?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error + Send + Sync>> {
    // let keys: Vec<String> = db.keys("player:*")?; // TODO use SCAN instead
    // db.del(keys)?;
    let members: Vec<String> = db.smembers(KEY_PLAYER_INDEX)?;
    for member in members.iter() {
        delete_by_id(db, member)?;
    }
    db.del(KEY_PLAYER_INDEX)?;
    Ok(())
}
pub fn delete(
    db: &mut redis::Connection,
    player: &Player,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    delete_by_id(db, &player.id)
}
pub fn delete_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.del(get_key_player(&id))?;
    db.srem(KEY_PLAYER_INDEX, &id)?;
    Ok(())
}
pub fn set_lobby(
    db: &mut redis::Connection,
    player: &Player,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(&player.id), KEY_LOBBY, &player.lobby)?;
    Ok(())
}
pub fn set_lobby_by_id(
    db: &mut redis::Connection,
    id: &String,
    lobby: &String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(&id), KEY_LOBBY, &lobby)?;
    Ok(())
}
pub fn set_game(
    db: &mut redis::Connection,
    player: &Player,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    set_game_by_id(db, &player.id, &player.game)
}
pub fn set_game_by_id(
    db: &mut redis::Connection,
    id: &String,
    game: &String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(&id), KEY_GAME, &game)?;
    Ok(())
}
pub fn set_mmr(
    db: &mut redis::Connection,
    player: &Player,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(&player.id), KEY_MMR, &player.mmr)?;
    Ok(())
}
pub fn set_state(
    db: &mut redis::Connection,
    player: &Player,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    set_state_by_id(db, &player.id, player.state())
}
pub fn set_state_by_id(
    db: &mut redis::Connection,
    player_id: &String,
    state: PlayerState,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(player_id), KEY_STATE, &(state as i32))?;
    Ok(())
}

// Gets
pub fn get_index(db: &mut redis::Connection) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let members: Vec<String> = db.smembers(KEY_PLAYER_INDEX)?;
    Ok(members)
}
pub fn get(
    db: &mut redis::Connection,
    id: &String,
) -> Result<Player, Box<dyn Error + Send + Sync>> {
    let mut player = Player::default();
    player.id = id.clone();
    get_lobby(db, &mut player)?;
    get_game(db, &mut player)?;
    get_mmr(db, &mut player)?;
    get_state(db, &mut player)?;
    Ok(player)
}
pub fn get_lobby(
    db: &mut redis::Connection,
    player: &mut Player,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    player.lobby = get_lobby_by_id(db, &player.id)?;
    Ok(())
}
pub fn get_game(
    db: &mut redis::Connection,
    player: &mut Player,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    player.lobby = get_game_by_id(db, &player.id)?;
    Ok(())
}
pub fn get_mmr(
    db: &mut redis::Connection,
    player: &mut Player,
) -> Result<u32, Box<dyn Error + Send + Sync>> {
    player.mmr = get_mmr_by_id(db, &player.id)?;
    Ok(player.mmr)
}
pub fn get_state(
    db: &mut redis::Connection,
    player: &mut Player,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    player.state = get_state_by_id(db, &player.id)?;
    Ok(player.state)
}
pub fn get_lobby_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let lobby = db.hget(get_key_player(&id), KEY_LOBBY)?;
    Ok(lobby)
}
pub fn get_game_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let lobby = db.hget(get_key_player(&id), KEY_GAME)?;
    Ok(lobby)
}
pub fn get_mmr_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<u32, Box<dyn Error + Send + Sync>> {
    let mmr = db.hget(get_key_player(&id), KEY_MMR)?;
    Ok(mmr)
}
pub fn get_state_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let state = db.hget(get_key_player(&id), KEY_STATE)?;
    Ok(state)
}
