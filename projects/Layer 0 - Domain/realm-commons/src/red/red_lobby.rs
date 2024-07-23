use crate::protos::models::Lobby;
use redis::Commands;
use std::{error::Error, num::NonZeroUsize};

// Keys
fn get_key_lobby(lobby: &String) -> String {
    "lobby:".to_string() + lobby
}
fn get_key_lobby_queue(lobby: &String) -> String {
    let mut str: String = get_key_lobby(lobby);
    str.push_str(":queue");
    str
}
fn get_key_lobby_state(lobby: &String) -> String {
    let mut str: String = get_key_lobby(lobby);
    str.push_str(":state");
    str
}
fn get_key_lobby_players(lobby: &String) -> String {
    let mut str: String = get_key_lobby(lobby);
    str.push_str(":players");
    str
}

// Sets
pub fn set(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    let _ = set_queue(db, lobby);
    let _ = set_state(db, lobby);
    let _ = set_players(db, lobby);
    Ok(())
}
pub fn delete(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    db.del(get_key_lobby(&lobby.id))?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error>> {
    let keys: Vec<String> = db.keys("lobby:*")?;
    db.del(keys)?;
    Ok(())
}
pub fn set_queue(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    db.set(get_key_lobby_queue(&lobby.id), lobby.queue)?;
    Ok(())
}
pub fn set_state(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    db.set(get_key_lobby_state(&lobby.id), lobby.state)?;
    Ok(())
}
pub fn set_players(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error>> {
    let key = get_key_lobby_players(&lobby.id);
    let len = db.llen("key")?;
    let opt_len = NonZeroUsize::new(len);
    db.lpop(&key, opt_len)?;
    db.lpush(&key, &lobby.players)?;
    Ok(())
}

// Gets
pub fn get(db: &mut redis::Connection, id: &String) -> Result<Lobby, Box<dyn Error>> {
    let mut lobby = Lobby::default();
    lobby.id = id.clone();
    get_queue(db, &mut lobby)?;
    get_state(db, &mut lobby)?;
    get_players(db, &mut lobby)?;
    Ok(lobby)
}
pub fn get_queue(db: &mut redis::Connection, lobby: &mut Lobby) -> Result<i32, Box<dyn Error>> {
    lobby.queue = get_queue_by_id(db, &lobby.id)?;
    Ok(lobby.queue)
}
pub fn get_state(db: &mut redis::Connection, lobby: &mut Lobby) -> Result<i32, Box<dyn Error>> {
    lobby.state = get_state_by_id(db, &lobby.id)?;
    Ok(lobby.state)
}
pub fn get_players(db: &mut redis::Connection, lobby: &mut Lobby) -> Result<(), Box<dyn Error>> {
    lobby.players = get_players_by_id(db, &lobby.id)?;
    Ok(())
}
pub fn get_queue_by_id(db: &mut redis::Connection, id: &String) -> Result<i32, Box<dyn Error>> {
    let queue = db.get(get_key_lobby_queue(id))?;
    Ok(queue)
}
pub fn get_state_by_id(db: &mut redis::Connection, id: &String) -> Result<i32, Box<dyn Error>> {
    let state = db.get(get_key_lobby_state(id))?;
    Ok(state)
}
pub fn get_players_by_id(db: &mut redis::Connection, id: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let players = db.lrange(get_key_lobby_players(id), 0, -1)?;
    Ok(players)
}

