use std::error::Error;

use redis::Commands;

use crate::protos::models::Player;

// Keys
fn get_key_player(player: &String) -> String {
    "player:".to_string() + player
}
fn get_key_player_lobby(player: &String) -> String {
    let mut str: String = get_key_player(player);
    str.push_str(":lobby");
    str
}
fn get_key_player_mmr(player: &String) -> String {
    let mut str: String = get_key_player(player);
    str.push_str(":mmr");
    str
}
fn get_key_player_state(player: &String) -> String {
    let mut str: String = get_key_player(player);
    str.push_str(":state");
    str
}

// Values
pub fn set(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    set_lobby(db, player)?;
    set_mmr(db, player)?;
    set_state(db, player)?;
    Ok(())
}
pub fn delete(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.del(get_key_player(&player.id))?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let keys: Vec<String> = db.keys("player:*")?;
    db.del(keys)?;
    Ok(())
}
pub fn set_lobby(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.set(get_key_player_lobby(&player.id), &player.lobby)?;
    Ok(())
}
pub fn set_mmr(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.set(get_key_player_mmr(&player.id), player.mmr)?;
    Ok(())
}
pub fn set_state(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.set(get_key_player_state(&player.id), player.state)?;
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
    let lobby = db.get(get_key_player_lobby(id))?;
    Ok(lobby)
}

pub fn get_mmr_by_id(db: &mut redis::Connection, id: &String) -> Result<u32, Box<dyn Error + Send + Sync>> {
    let mmr = db.get(get_key_player_mmr(id))?;
    Ok(mmr)
}
pub fn get_state_by_id(db: &mut redis::Connection, id: &String) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let state = db.get(get_key_player_state(id))?;
    Ok(state)
}
