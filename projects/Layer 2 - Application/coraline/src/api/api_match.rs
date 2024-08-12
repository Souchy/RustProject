use crate::CORALINE;
use realm_commons::protos::models::Lobby;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings:
        get,
        win,
        lose
    ]
}

#[openapi(tag = "Match")]
#[get("/")]
async fn get() -> Json<Option<Lobby>> {
    let mut coraline = CORALINE.lock().await;
    let player_id = coraline.player.id.clone();
    // TODO
    // if let Some(db) = &mut coraline.db {
    //     let player = red_player::get(db, &player_id).ok();
    //     let lobby = red_lobby::get(db, &player.unwrap().lobby).ok();
    //     return Json(lobby);
    // }
    return Json(None);
}

#[openapi(tag = "Match")]
#[post("/win")]
async fn win() {
    let coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();
    // TODO: send win match packet to coral
}

#[openapi(tag = "Match")]
#[post("/lose")]
async fn lose() {
    let coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();
    // TODO: send lose match packet to coral
}
