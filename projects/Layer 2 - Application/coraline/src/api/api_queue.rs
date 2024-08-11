use coral_commons::protos::messages::{QueueType, SetQueueRequest};

use realm_commons::{
    protos::models::Lobby,
    red::{red_lobby, red_player},
};
use rocket::{form::FromForm, post, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes};
use serde::{Deserialize, Serialize};
use teal::net::message;

use crate::CORALINE;

pub fn get_routes() -> Vec<rocket::Route> {
    openapi_get_routes![
        get,
        set_queue,
        enter_queue_normal,
        enter_queue_ranked,
        exit_queue
    ]
}

// Request models
#[derive(Serialize, Deserialize, FromForm, ::schemars::JsonSchema)]
struct SetLobbyQueueModel {
    queue: QueueType,
}
impl Default for SetLobbyQueueModel {
    fn default() -> SetLobbyQueueModel {
        SetLobbyQueueModel {
            queue: QueueType::Normal,
        }
    }
}

#[openapi(tag = "Queue")]
#[post("/")]
async fn get() -> Json<Option<Lobby>> {
    let mut coraline = CORALINE.lock().await;
    let player_id = coraline.player.id.clone();

    if let Some(db) = &mut coraline.db {
        let player = red_player::get(db, &player_id).ok();
        let lobby = red_lobby::get(db, &player.unwrap().lobby).ok();
        return Json(lobby);
    }
    return Json(None);
}

#[openapi(tag = "Queue")]
#[post("/set_queue", data = "<json>")]
async fn set_queue(json: Json<SetLobbyQueueModel>) {
    let coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();

    let lobby_id = coraline.player.lobby.clone();
    let req = SetQueueRequest {
        lobby: lobby_id,
        queue: json.queue as i32,
    };

    let buf = message::serialize(&req);
    let _ = client_ref.send_bytes(&buf).await;
}

#[openapi(tag = "Queue")]
#[post("/enter_queue_normal")]
async fn enter_queue_normal() {
    let coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();

    let lobby_id = coraline.player.lobby.clone();
    let req = SetQueueRequest {
        lobby: lobby_id,
        queue: QueueType::Normal as i32,
    };

    let buf = message::serialize(&req);
    let _ = client_ref.send_bytes(&buf).await;
}

#[openapi(tag = "Queue")]
#[post("/enter_queue_ranked")]
async fn enter_queue_ranked() {
    let coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();

    let lobby_id = coraline.player.lobby.clone();
    let req = SetQueueRequest {
        lobby: lobby_id,
        queue: QueueType::Ranked as i32,
    };

    let buf = message::serialize(&req);
    let _ = client_ref.send_bytes(&buf).await;
}

#[openapi(tag = "Queue")]
#[post("/exit_queue", data = "<json>")]
async fn exit_queue(json: Json<SetLobbyQueueModel>) {
    let coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();

    let lobby_id = coraline.player.lobby.clone();

    let req = SetQueueRequest {
        lobby: lobby_id,
        queue: QueueType::Idle as i32,
    };

    let buf = message::serialize(&req);
    let _ = client_ref.send_bytes(&buf).await;
}
