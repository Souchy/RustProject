use realm_commons::{protos::models::Player, red::red_player};
use rocket::{get, serde::json::Json};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes, openapi_get_routes_spec,
    settings::OpenApiSettings,
};

use crate::CORALINE;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get]
}

#[openapi(tag = "Player")]
#[get("/")]
async fn get() -> Json<Option<Player>> {
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
