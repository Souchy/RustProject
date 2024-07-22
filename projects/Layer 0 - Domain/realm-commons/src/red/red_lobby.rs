use crate::protos::models::Lobby;
use redis::Commands;
use std::{error::Error, num::NonZeroUsize};

// Keys
fn get_key_lobby(lobby: &Lobby) -> String {
    let mut str: String = "lobby:".to_string();
    str.push_str(&lobby.id.to_string());
    str
}
fn get_key_lobby_queue(lobby: &Lobby) -> String {
    let mut str: String = get_key_lobby(lobby);
    str.push_str(":queue");
    str
}

fn get_key_lobby_state(lobby: &Lobby) -> String {
    let mut str: String = get_key_lobby(lobby);
    str.push_str(":state");
    str
}

fn get_key_lobby_players(lobby: &Lobby) -> String {
    let mut str: String = get_key_lobby(lobby);
    str.push_str(":players");
    str
}

// Values
pub fn set(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    let _ = set_queue(db, lobby);
    let _ = set_state(db, lobby);
    let _ = set_players(db, lobby);
    Ok(())
}
pub fn delete(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    db.del(get_key_lobby(lobby))?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error>> {
    let keys: Vec<String> = db.keys("lobby:*")?;
    db.del(keys)?;
    Ok(())
}
pub fn set_queue(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    db.set(get_key_lobby_queue(&lobby), lobby.queue)?;
    Ok(())
}
pub fn set_state(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    db.set(get_key_lobby_state(&lobby), lobby.state)?;
    Ok(())
}
pub fn set_players(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    let key = get_key_lobby_players(&lobby);
    let len = db.llen("key")?;
    let opt_len = NonZeroUsize::new(len);
    db.lpop(&key, opt_len)?;
    db.lpush(&key, &lobby.players)?;
    Ok(())
}
