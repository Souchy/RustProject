use redis::Commands;
use std::{collections::HashMap, error::Error, num::NonZeroUsize};

use crate::protos::models::{Match, MatchState};

const KEY_MATCH_INDEX: &str = "match_ids";

// Keys
const KEY_MATCH: &str = "match";
const KEY_QUEUE: &str = "queue";
const KEY_PORT: &str = "port";
const KEY_DATE: &str = "date";
const KEY_STATE: &str = "state";
const KEY_PLAYERS: &str = "players";
fn get_key_match(r#match: &String) -> String {
    KEY_MATCH.to_string() + ":" + r#match
}
fn get_key_match_players(r#match: &String) -> String {
    let mut str: String = get_key_match(r#match);
    str.push_str(":");
    str.push_str(KEY_PLAYERS);
    str
}

// Sets
pub fn set(
    db: &mut redis::Connection,
    r#match: &Match,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    set_queue(db, &r#match)?;
    set_port(db, &r#match)?;
    set_players(db, &r#match)?;
    set_date(db, &r#match)?;
    set_state(db, &r#match)?;
    db.sadd(KEY_MATCH_INDEX, &r#match.id)?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let members: Vec<String> = db.smembers(KEY_MATCH_INDEX)?;
    for member in members.iter() {
        delete_by_id(db, member)?;
    }
    db.del(KEY_MATCH_INDEX)?;
    Ok(())
}
pub fn delete(
    db: &mut redis::Connection,
    r#match: &Match,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    delete_by_id(db, &r#match.id)
}
pub fn delete_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.del(get_key_match(&id))?;
    db.srem(KEY_MATCH_INDEX, &id)?;
    Ok(())
}
pub fn set_queue(
    db: &mut redis::Connection,
    r#match: &Match,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_match(&r#match.id), KEY_QUEUE, r#match.queue)?;
    Ok(())
}
pub fn set_port(
    db: &mut redis::Connection,
    r#match: &Match,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_match(&r#match.id), KEY_PORT, r#match.game_port)?;
    Ok(())
}
pub fn set_date(
    db: &mut redis::Connection,
    r#match: &Match,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_match(&r#match.id), KEY_DATE, r#match.date)?;
    Ok(())
}
pub fn set_state(
    db: &mut redis::Connection,
    r#match: &Match,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.hset(get_key_match(&r#match.id), KEY_STATE, r#match.state)?;
    Ok(())
}
pub fn set_players(
    db: &mut redis::Connection,
    r#match: &Match,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let key = get_key_match_players(&r#match.id);
    db.del(&key)?;
    for p in &r#match.players {
        db.hset(&key, p.0, p.1)?;
    }
    Ok(())
}

// Gets
pub fn get_index(db: &mut redis::Connection) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let members: Vec<String> = db.smembers(KEY_MATCH_INDEX)?;
    Ok(members)
}
pub fn get(db: &mut redis::Connection, id: &String) -> Result<Match, Box<dyn Error + Send + Sync>> {
    let mut r#match = Match::default();
    r#match.id = id.clone();
    get_queue(db, &mut r#match)?;
    get_port(db, &mut r#match)?;
    get_players(db, &mut r#match)?;
    get_date(db, &mut r#match)?;
    get_state(db, &mut r#match)?;
    Ok(r#match)
}
pub fn get_queue(
    db: &mut redis::Connection,
    r#match: &mut Match,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    r#match.queue = get_queue_by_id(db, &r#match.id)?;
    Ok(r#match.queue)
}
pub fn get_port(
    db: &mut redis::Connection,
    r#match: &mut Match,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    r#match.game_port = get_port_by_id(db, &r#match.id)?;
    Ok(r#match.game_port)
}
pub fn get_date(
    db: &mut redis::Connection,
    r#match: &mut Match,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    r#match.date = get_date_by_id(db, &r#match.id)?;
    Ok(r#match.date)
}
pub fn get_state(
    db: &mut redis::Connection,
    r#match: &mut Match,
) -> Result<MatchState, Box<dyn Error + Send + Sync>> {
    r#match.state = get_state_by_id(db, &r#match.id)?;
    Ok(r#match.state())
}
pub fn get_players(
    db: &mut redis::Connection,
    r#match: &mut Match,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    r#match.players = get_players_by_id(db, &r#match.id)?;
    Ok(())
}

pub fn get_queue_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_match(id), KEY_QUEUE)?;
    Ok(val)
}
pub fn get_port_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_match(id), KEY_PORT)?;
    Ok(val)
}
pub fn get_date_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_match(id), KEY_DATE)?;
    Ok(val)
}
pub fn get_state_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let val = db.hget(get_key_match(id), KEY_STATE)?;
    Ok(val)
}
pub fn get_players_by_id(
    db: &mut redis::Connection,
    id: &String,
) -> Result<HashMap<String, String>, Box<dyn Error + Send + Sync>> {
    let val = db.hgetall(get_key_match_players(id))?;
    Ok(val)
}
