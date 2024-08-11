use crate::protos::models::Lobby;
use redis::{cmd, Commands, ConnectionLike};
use std::{error::Error, num::NonZeroUsize};

// Keys
fn get_key_lobby(lobby: &String) -> String {
    "lobby:".to_string() + lobby
}
const KEY_QUEUE: &str = "queue";
const KEY_STATE: &str = "state";
const KEY_AVERAGE_MMR: &str = "average_mmr";
fn get_key_lobby_players(lobby: &String) -> String {
    let mut str: String = get_key_lobby(lobby);
    str.push_str(":players");
    str
}

// Sets
pub fn set(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error + Send + Sync>> {
    set_queue(db, lobby)?;
    set_state(db, lobby)?;
    set_players(db, lobby)?;
    set_average_mmr(db, lobby)?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let keys: Vec<String> = db.keys("lobby:*")?;
    db.del(keys)?;
    Ok(())
}
pub fn delete(
    db: &mut redis::Connection,
    lobby: &Lobby,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    delete_by_id(db, &lobby.id)
}
pub fn delete_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // db.del(get_key_lobby_queue(&id))?;
    // db.del(get_key_lobby_state(&id))?;
    db.del(get_key_lobby_players(&id))?;
    // db.del(get_key_lobby_average_mmr(&id))?;
    db.del(get_key_lobby(&id))?;
    Ok(())
}
pub fn set_queue(
    db: &mut redis::Connection,
    lobby: &Lobby,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_lobby(&lobby.id), KEY_QUEUE, &lobby.queue)?;
    Ok(())
}
pub fn set_state(
    db: &mut redis::Connection,
    lobby: &Lobby,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_lobby(&lobby.id), KEY_STATE, &lobby.state)?;
    Ok(())
}
pub fn set_average_mmr(
    db: &mut redis::Connection,
    lobby: &Lobby,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(
        get_key_lobby(&lobby.id),
        KEY_AVERAGE_MMR,
        &lobby.average_mmr,
    )?;
    Ok(())
}
pub fn set_players(
    db: &mut redis::Connection,
    lobby: &Lobby,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let key = get_key_lobby_players(&lobby.id);
    let len = db.llen(&key)?;
    let opt_len = NonZeroUsize::new(len);
    db.lpop(&key, opt_len)?;
    db.lpush(&key, &lobby.players)?;
    Ok(())
}

// Gets
pub fn get(db: &mut redis::Connection, id: &String) -> Result<Lobby, Box<dyn Error + Send + Sync>> {
    let mut lobby = Lobby::default();
    lobby.id = id.clone();
    get_queue(db, &mut lobby)?;
    get_state(db, &mut lobby)?;
    get_players(db, &mut lobby)?;
    get_average_mmr(db, &mut lobby)?;
    Ok(lobby)
}
// TODO get lobbies
pub fn get_ids(db: &mut redis::Connection) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let _ds = db.scan_match::<&str, String>("lobby:*")?;

    // let keys = db.scan(0, "lobby:*:".to_string())?;
    // return keys;
    Ok(Vec::new())
}
pub fn get_queue(
    db: &mut redis::Connection,
    lobby: &mut Lobby,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    lobby.queue = get_queue_by_id(db, &lobby.id)?;
    Ok(lobby.queue)
}
pub fn get_state(
    db: &mut redis::Connection,
    lobby: &mut Lobby,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    lobby.state = get_state_by_id(db, &lobby.id)?;
    Ok(lobby.state)
}
pub fn get_players(
    db: &mut redis::Connection,
    lobby: &mut Lobby,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    lobby.players = get_players_by_id(db, &lobby.id)?;
    Ok(())
}
pub fn get_average_mmr(
    db: &mut redis::Connection,
    lobby: &mut Lobby,
) -> Result<u32, Box<dyn Error + Send + Sync>> {
    lobby.average_mmr = get_average_mmr_by_id(db, &lobby.id)?;
    Ok(lobby.average_mmr)
}
pub fn get_queue_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let queue = db.hget(get_key_lobby(&id), KEY_QUEUE)?;
    Ok(queue)
}
pub fn get_state_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let state = db.hget(get_key_lobby(&id), KEY_STATE)?;
    Ok(state)
}
pub fn get_average_mmr_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<u32, Box<dyn Error + Send + Sync>> {
    let mmr = db.hget(get_key_lobby(&id), KEY_AVERAGE_MMR)?;
    Ok(mmr)
}
pub fn get_players_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let players = db.lrange(get_key_lobby_players(id), 0, -1)?;
    Ok(players)
}

// Queries
pub fn find_lobby_match(db: &mut redis::Connection, lobby1: &Lobby) {}

pub fn create_index(db: &mut redis::Connection) {
    let mut command = cmd("HDEL");
    let full = command.arg("key").arg("field");
    let res = db.req_command(&full);
    
}
