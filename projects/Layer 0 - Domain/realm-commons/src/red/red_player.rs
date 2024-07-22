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
pub fn set(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error>> {
    let _ = set_lobby(db, player);
    let _ = set_mmr(db, player);
    let _ = set_state(db, player);
    Ok(())
}
pub fn delete(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error>> {
    db.del(get_key_player(&player.id))?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error>> {
    let keys: Vec<String> = db.keys("player:*")?;
    db.del(keys)?;
    Ok(())
}
pub fn set_lobby(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error>> {
    db.set(get_key_player_lobby(&player.id), &player.lobby)?;
    Ok(())
}
pub fn set_mmr(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error>> {
    db.set(get_key_player_mmr(&player.id), player.mmr)?;
    Ok(())
}
pub fn set_state(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error>> {
    db.set(get_key_player_state(&player.id), player.state)?;
    Ok(())
}

// Gets
pub fn get(db: &mut redis::Connection, id: String) -> Result<Player, Box<dyn Error>> {
    let mut player = Player::default();
    player.id = id;
    let _ = get_lobby(db, &mut player);
    let _ = get_mmr(db, &mut player);
    let _ = get_state(db, &mut player);
    Ok(player)
}
pub fn get_lobby(db: &mut redis::Connection, player: &mut Player) -> Result<(), Box<dyn Error>> {
    player.lobby = get_lobby_by_id(db, &player.id)?;
    Ok(())
}
pub fn get_mmr(db: &mut redis::Connection, player: &mut Player) -> Result<i32, Box<dyn Error>> {
    player.mmr = get_mmr_by_id(db, &player.id)?;
    Ok(player.mmr)
}
pub fn get_state(db: &mut redis::Connection, player: &mut Player) -> Result<i32, Box<dyn Error>> {
    player.state = get_state_by_id(db, &player.id)?;
    Ok(player.state)
}
pub fn get_lobby_by_id(db: &mut redis::Connection, id: &String) -> Result<String, Box<dyn Error>> {
    let lobby = db.get(get_key_player_lobby(id))?;
    Ok(lobby)
}

pub fn get_mmr_by_id(db: &mut redis::Connection, id: &String) -> Result<i32, Box<dyn Error>> {
    let mmr = db.get(get_key_player_mmr(id))?;
    Ok(mmr)
}
pub fn get_state_by_id(db: &mut redis::Connection, id: &String) -> Result<i32, Box<dyn Error>> {
    let state = db.get(get_key_player_state(id))?;
    Ok(state)
}
