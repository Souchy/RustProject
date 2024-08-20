use realm_commons::{protos::models::Player, red::red_player};
use rocket::{get, serde::json::Json};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get]
}

#[openapi(tag = "Player")]
#[get("/")]
async fn get() -> Json<Option<Player>> {
    let mut coraline = crate::CORALINE.lock().await;
    let player_id = coraline.player_id.clone();
    if let Some(db) = &mut coraline.db {
        let player = red_player::get(db, &player_id).ok();
        return Json(player);
    }
    return Json(None);
}
