use redis::Commands;
use std::{error::Error, num::NonZeroUsize};

use crate::protos::models::Match;

// Keys
fn get_key_lobby(r#match: &String) -> String {
    "match:".to_string() + r#match
}
fn get_key_lobby_queue(r#match: &String) -> String {
    let mut str: String = get_key_lobby(r#match);
    str.push_str(":queue");
    str
}
fn get_key_lobby_game_port(r#match: &String) -> String {
    let mut str: String = get_key_lobby(r#match);
    str.push_str(":game_port");
    str
}
fn get_key_lobby_players(r#match: &String) -> String {
    let mut str: String = get_key_lobby(r#match);
    str.push_str(":players");
    str
}

// Sets
pub fn set(db: &mut redis::Connection, r#match: &Match) -> Result<(), Box<dyn Error>> {
    let _ = set_queue(db, r#match);
    let _ = set_game_port(db, r#match);
    let _ = set_players(db, r#match);
    Ok(())
}
pub fn delete(db: &mut redis::Connection, r#match: &Match) -> Result<(), Box<dyn Error>> {
    db.del(get_key_lobby(&r#match.id))?;
    Ok(())
}
pub fn delete_all(db: &mut redis::Connection) -> Result<(), Box<dyn Error>> {
    let keys: Vec<String> = db.keys("match:*")?;
    db.del(keys)?;
    Ok(())
}
pub fn set_queue(db: &mut redis::Connection, r#match: &Match) -> Result<(), Box<dyn Error>> {
    db.set(get_key_lobby_queue(&r#match.id), r#match.queue)?;
    Ok(())
}
pub fn set_game_port(db: &mut redis::Connection, r#match: &Match) -> Result<(), Box<dyn Error>> {
    db.set(get_key_lobby_game_port(&r#match.id), r#match.game_port)?;
    Ok(())
}
pub fn set_players(db: &mut redis::Connection, r#match: &Match) -> Result<(), Box<dyn Error>> {
    let key = get_key_lobby_players(&r#match.id);
    let len = db.llen("key")?;
    let opt_len = NonZeroUsize::new(len);
    db.lpop(&key, opt_len)?;
    db.lpush(&key, &r#match.players)?;
    Ok(())
}

// Gets
pub fn get(db: &mut redis::Connection, id: String) -> Result<Match, Box<dyn Error>> {
    let mut r#match = Match::default();
    r#match.id = id;
    let _ = get_queue(db, &mut r#match);
    let _ = get_game_port(db, &mut r#match);
    let _ = get_players(db, &mut r#match);
    Ok(r#match)
}
pub fn get_queue(db: &mut redis::Connection, r#match: &mut Match) -> Result<i32, Box<dyn Error>> {
    r#match.queue = get_queue_by_id(db, &r#match.id)?;
    Ok(r#match.queue)
}
pub fn get_game_port(db: &mut redis::Connection, r#match: &mut Match) -> Result<i32, Box<dyn Error>> {
    r#match.game_port = get_game_port_by_id(db, &r#match.id)?;
    Ok(r#match.game_port)
}
pub fn get_players(db: &mut redis::Connection, r#match: &mut Match) -> Result<(), Box<dyn Error>> {
    r#match.players = get_players_by_id(db, &r#match.id)?;
    Ok(())
}

pub fn get_queue_by_id(db: &mut redis::Connection, id: &String) -> Result<i32, Box<dyn Error>> {
    let queue = db.get(get_key_lobby_queue(id))?;
    Ok(queue)
}
pub fn get_game_port_by_id(db: &mut redis::Connection, id: &String) -> Result<i32, Box<dyn Error>> {
    let game_port = db.get(get_key_lobby_game_port(id))?;
    Ok(game_port)
}
pub fn get_players_by_id(db: &mut redis::Connection, id: &String) -> Result<Vec<i64>, Box<dyn Error>> {
    let players = db.get(get_key_lobby_players(id))?;
    Ok(players)
}

