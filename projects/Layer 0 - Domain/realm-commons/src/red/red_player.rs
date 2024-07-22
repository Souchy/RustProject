use std::error::Error;

use redis::Commands;

use crate::protos::models::Player;

// Keys
fn get_key_player(player: &Player) -> String {
    let mut str: String = "player:".to_string();
    str.push_str(&player.id.to_string());
    str
}
fn get_key_player_lobby(player: &Player) -> String {
    let mut str: String = get_key_player(player);
    str.push_str(":lobby");
    str
}

fn get_key_player_mmr(player: &Player) -> String {
    let mut str: String = get_key_player(player);
    str.push_str(":mmr");
    str
}

fn get_key_player_state(player: &Player) -> String {
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
    db.del(get_key_player(player))?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error>> {
    let keys: Vec<String> = db.keys("player:*")?;
    db.del(keys)?;
    Ok(())
}
pub fn set_lobby(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error>> {
    db.set(get_key_player_lobby(&player), player.lobby)?;
    Ok(())
}
pub fn set_mmr(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error>> {
    db.set(get_key_player_mmr(&player), player.mmr)?;
    Ok(())
}
pub fn set_state(db: &mut redis::Connection, player: &Player) -> Result<(), Box<dyn Error>> {
    db.set(get_key_player_state(&player), player.state)?;
    Ok(())
}
