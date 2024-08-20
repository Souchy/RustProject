use realm_commons::protos::models::Player;
use realm_commons::red::red_player;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get, get_all, delete_all]
}

#[openapi(tag = "Players")]
#[get("/<id>")]
async fn get(id: String) -> Json<Option<Player>> {
    unsafe {
        if let Some(db) = &mut crate::DB {
            let opt_player = red_player::get(db, &id).ok();
            return Json(opt_player);
        }
    }
    return Json(None);
}

#[openapi(tag = "Players")]
#[get("/all")]
async fn get_all() -> Json<Vec<Player>> {
    unsafe {
        if let Some(db) = &mut crate::DB {
            let opt_index = red_player::get_index(db).ok();
            let mut players: Vec<Player> = Vec::new();
            if let Some(index) = opt_index {
                for id in index.iter() {
                    let opt_player = red_player::get(db, id).ok();
                    if let Some(player) = opt_player {
                        players.push(player);
                    }
                }
                return Json(players);
            }
        }
    }
    return Json(vec![]);
}

#[openapi(tag = "Players")]
#[post("/delete_all")]
async fn delete_all() {
    unsafe {
        if let Some(db) = &mut crate::DB {
            red_player::delete_all(db).ok();
        }
    }
}
