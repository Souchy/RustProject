use std::error::Error;

use redis::Commands;

use crate::protos::models::{player::PlayerState, Player};

// Keys
fn get_key_player(player: &String) -> String {
    "player:".to_string() + player
}
const KEY_STATE: &str = "state";
const KEY_LOBBY: &str = "lobby";
const KEY_MMR: &str = "mmr";

// Values
pub fn set(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    set_lobby(db, player)?;
    set_mmr(db, player)?;
    set_state(db, player)?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let keys: Vec<String> = db.keys("player:*")?; // TODO use SCAN instead
    db.del(keys)?;
    Ok(())
}
pub fn delete(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    delete_by_id(db, &player.id)
}
pub fn delete_by_id(db: &mut redis::Connection, id: &String) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.del(get_key_player(&id))?;
    Ok(())
}
pub fn set_lobby(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(&player.id), KEY_LOBBY, &player.lobby)?;
    Ok(())
}
pub fn set_mmr(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(&player.id), KEY_MMR, &player.mmr)?;
    Ok(())
}
pub fn set_state(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(&player.id), KEY_STATE, &player.state)?;
    Ok(())
}
pub fn set_state_by_id(db: &mut redis::Connection, player_id: &String, state: PlayerState) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_player(player_id), KEY_STATE, &(state as i32))?;
    Ok(())
}

// Gets
pub fn get(db: &mut redis::Connection, id: &String) -> Result<Player, Box<dyn Error + Send + Sync>> {
    let mut player = Player::default();
    player.id = id.clone();
    get_lobby(db, &mut player)?;
    get_mmr(db, &mut player)?;
    get_state(db, &mut player)?;
    Ok(player)
}
pub fn get_lobby(db: &mut redis::Connection, player: &mut Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    player.lobby = get_lobby_by_id(db, &player.id)?;
    Ok(())
}
pub fn get_mmr(db: &mut redis::Connection, player: &mut Player) -> Result<u32, Box<dyn Error + Send + Sync>> {
    player.mmr = get_mmr_by_id(db, &player.id)?;
    Ok(player.mmr)
}
pub fn get_state(db: &mut redis::Connection, player: &mut Player) -> Result<i32, Box<dyn Error + Send + Sync>> {
    player.state = get_state_by_id(db, &player.id)?;
    Ok(player.state)
}
pub fn get_lobby_by_id(db: &mut redis::Connection, id: &String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let lobby = db.hget(get_key_player(&id), KEY_LOBBY)?;
    Ok(lobby)
}

pub fn get_mmr_by_id(db: &mut redis::Connection, id: &String) -> Result<u32, Box<dyn Error + Send + Sync>> {
    let mmr = db.hget(get_key_player(&id), KEY_MMR)?;
    Ok(mmr)
}
pub fn get_state_by_id(db: &mut redis::Connection, id: &String) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let state = db.hget(get_key_player(&id), KEY_STATE)?;
    Ok(state)
}
