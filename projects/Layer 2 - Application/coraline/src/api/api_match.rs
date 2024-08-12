use crate::CORALINE;
use coral_commons::{
    protos::models::{Match, MatchResult, MatchResultType},
    red::red_match,
};
use realm_commons::{protos::models::Lobby, red::red_player};
use rocket::{get, post, serde::json::Json};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};
use teal::net::message;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings:
        get,
        win,
        lose
    ]
}

#[openapi(tag = "Match")]
#[get("/")]
async fn get() -> Json<Option<Match>> {
    let mut coraline = CORALINE.lock().await;
    let player_id = coraline.player_id.clone();

    if let Some(db) = &mut coraline.db {
        let player = red_player::get(db, &player_id).ok();
        let game = red_match::get(db, &player.unwrap().game).ok();
        return Json(game);
    }
    return Json(None);
}

#[openapi(tag = "Match")]
#[post("/win")]
async fn win() {
    let mut coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();
    let player_id = coraline.player_id.clone();

    if let Some(db) = &mut coraline.db {
        let player = red_player::get(db, &player_id).unwrap();
        let match_result = MatchResult {
            id: player.game,
            result: MatchResultType::Win as i32,
        };
        let buf = message::serialize(&match_result);
        client_ref.send_bytes(&buf).await.ok();
    }
}

#[openapi(tag = "Match")]
#[post("/lose")]
async fn lose() {
    let mut coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();
    let player_id = coraline.player_id.clone();

    if let Some(db) = &mut coraline.db {
        let player = red_player::get(db, &player_id).unwrap();
        let match_result = MatchResult {
            id: player.game,
            result: MatchResultType::Lose as i32,
        };
        let buf = message::serialize(&match_result);
        client_ref.send_bytes(&buf).await.ok();
    }
}
