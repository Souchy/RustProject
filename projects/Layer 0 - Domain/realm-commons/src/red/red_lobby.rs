use crate::protos::models::Lobby;
use redis::{cmd, Commands, ConnectionLike};
use std::{
    error::Error,
    num::NonZeroUsize,
    time::{SystemTime, UNIX_EPOCH},
};

// Keys
fn get_key_lobby(lobby: &String) -> String {
    "lobby:".to_string() + lobby
}
const KEY_TOKEN: &str = "token";
const KEY_QUEUE: &str = "queue";
const KEY_QUEUE_START_TIME: &str = "queue_start_time";
const KEY_STATE: &str = "state";
const KEY_AVERAGE_MMR: &str = "average_mmr";
const KEY_INDEX_MMR: &str = "queue_lobby_mmr";
fn get_key_lobby_players(lobby: &String) -> String {
    let mut str: String = get_key_lobby(lobby);
    str.push_str(":players");
    str
}
fn get_key_queue_lobby_mmr(queue: &i32) -> String {
    let mut str: String = KEY_INDEX_MMR.to_string();
    str.push_str(":");
    str.push_str(&queue.to_string());
    str
}

// Sets
pub fn set(db: &mut redis::Connection, lobby: &Lobby) -> Result<(), Box<dyn Error + Send + Sync>> {
    set_queue(db, lobby)?;
    set_queue_start_time(db, lobby)?;
    set_state(db, lobby)?;
    set_players(db, lobby)?;
    set_average_mmr(db, lobby)?;
    set_mmr_index(db, lobby)?;
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
    db.del(get_key_lobby(&id))?;
    db.del(get_key_lobby_players(&id))?;
    db.zrem(KEY_INDEX_MMR, &id)?;
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
pub fn set_queue_start_time(
    db: &mut redis::Connection,
    lobby: &Lobby,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_lobby(&lobby.id), KEY_QUEUE_START_TIME, &lobby.queue_start_time)?;
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
pub fn set_mmr_index(
    db: &mut redis::Connection,
    lobby: &Lobby,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.zadd(KEY_INDEX_MMR, &lobby.id, &lobby.average_mmr)?;
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
    get_queue_start_time(db, &mut lobby)?;
    get_state(db, &mut lobby)?;
    get_players(db, &mut lobby)?;
    get_average_mmr(db, &mut lobby)?;
    Ok(lobby)
}
pub fn get_token(
    db: &mut redis::Connection,
    lobby: &mut Lobby,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    lobby.token = get_token_by_id(db, &lobby.id)?;
    Ok(lobby.token.clone())
}
pub fn get_queue(
    db: &mut redis::Connection,
    lobby: &mut Lobby,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    lobby.queue = get_queue_by_id(db, &lobby.id)?;
    Ok(lobby.queue)
}
pub fn get_queue_start_time(
    db: &mut redis::Connection,
    lobby: &mut Lobby,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    lobby.queue_start_time = get_queue_start_time_by_id(db, &lobby.id)?;
    Ok(lobby.queue_start_time)
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

// Gets by id
pub fn get_token_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_lobby(&id), KEY_TOKEN)?;
    Ok(val)
}
pub fn get_queue_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_lobby(&id), KEY_QUEUE)?;
    Ok(val)
}
pub fn get_queue_start_time_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_lobby(&id), KEY_QUEUE_START_TIME)?;
    Ok(val)
}
pub fn get_state_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_lobby(&id), KEY_STATE)?;
    Ok(val)
}
pub fn get_average_mmr_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<u32, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_lobby(&id), KEY_AVERAGE_MMR)?;
    Ok(val)
}
pub fn get_players_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let val = db.lrange(get_key_lobby_players(id), 0, -1)?;
    Ok(val)
}

// Queries
pub fn find_lobby_match(
    db: &mut redis::Connection,
    lobby1: &Lobby,
) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
    // Increase mmr search range by 10 every 30 seconds
    const MMR_PER_SEC: u64 = 10 / 30;
    let mut offset: u32 = 100;

    let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let time_passed = time.as_secs() - lobby1.queue_start_time;
    offset += (time_passed * MMR_PER_SEC) as u32;

    let min = lobby1.average_mmr - offset;
    let max = lobby1.average_mmr + offset;

    let key = get_key_queue_lobby_mmr(&lobby1.queue);
    let range = db.zrangebyscore::<&str, u32, u32, Vec<String>>(&key, min, max)?;

    if range.len() == 0 {
        return Ok(None);
    }

    // TODO Use the first for now but should use a score based on the closest match and the time spent in queue for each lobby.
    // range.iter().min_by_key(|x| get_queue_start_time(x))

    Ok(Some(range[0].clone()))
}
