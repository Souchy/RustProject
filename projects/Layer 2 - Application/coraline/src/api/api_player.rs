use realm_commons::{protos::models::Player, red::red_player};
use rocket::{get, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes};

use crate::CORALINE;

pub fn get_routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_player]
}

#[openapi(tag = "Player")]
#[get("/")]
async fn get_player() -> Json<Option<Player>> {
    let mut coraline = CORALINE.lock().await;
    let player_id = coraline.player.id.clone();

    if let Some(db) = &mut coraline.db {
        let player = red_player::get(db, &player_id).ok();
        if let Some(p) = player.clone() {
            coraline.player = p;
            return Json(player);
        }
    }
    return Json(None);
}
