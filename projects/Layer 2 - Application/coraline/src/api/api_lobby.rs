use crate::CORALINE;
use coral_commons::protos::messages::{QueueType, SetQueueRequest};
use realm_commons::{
    protos::{client::CreateLobby, models::Lobby},
    red::{red_lobby, red_player},
};
use rocket::{form::FromForm, get, post, serde::json::Json};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes, openapi_get_routes_spec,
    settings::OpenApiSettings,
};
use serde::{Deserialize, Serialize};
use teal::net::message;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings:
        get,
        create_lobby,
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

#[openapi(tag = "Lobby")]
#[get("/")]
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


#[openapi(tag = "Lobby")]
#[post("/create_lobby")]
async fn create_lobby() {
    let coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();

    // Send a message to create a Lobby.
    // When it is created, we'll respond by setting the queue active.
    let create_lobby = CreateLobby { queue: QueueType::Idle as i32 };
    let create_lobby_buf = message::serialize(&create_lobby);
    client_ref.send_bytes(&create_lobby_buf).await.unwrap();
}

#[openapi(tag = "Lobby")]
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

#[openapi(tag = "Lobby")]
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
    client_ref.send_bytes(&buf).await.ok();
}

#[openapi(tag = "Lobby")]
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
    client_ref.send_bytes(&buf).await.ok();
}

#[openapi(tag = "Lobby")]
#[post("/exit_queue")]
async fn exit_queue() {
    let coraline = CORALINE.lock().await;
    let client_ref = coraline.client.clone().unwrap();

    let lobby_id = coraline.player.lobby.clone();

    let req = SetQueueRequest {
        lobby: lobby_id,
        queue: QueueType::Idle as i32,
    };

    let buf = message::serialize(&req);
    client_ref.send_bytes(&buf).await.ok();
}
