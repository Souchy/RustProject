use realm_commons::protos::models::Lobby;
use realm_commons::red::red_lobby;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get, get_all, delete_all]
}

#[openapi(tag = "Lobbies")]
#[get("/<id>")]
async fn get(id: String) -> Json<Option<Lobby>> {
    unsafe {
        if let Some(db) = &mut crate::DB {
            let opt_lobby = red_lobby::get(db, &id).ok();
            return Json(opt_lobby);
        }
    }
    return Json(None);
}

#[openapi(tag = "Lobbies")]
#[get("/all")]
async fn get_all() -> Json<Vec<Lobby>> {
    unsafe {
        if let Some(db) = &mut crate::DB {
            let opt_index = red_lobby::get_index(db).ok();
            let mut lobbies: Vec<Lobby> = Vec::new();
            if let Some(index) = opt_index {
                for id in index.iter() {
                    let opt_lobby = red_lobby::get(db, id).ok();
                    if let Some(lobby) = opt_lobby {
                        lobbies.push(lobby);
                    }
                }
                return Json(lobbies);
            }
        }
    }
    return Json(vec![]);
}

#[openapi(tag = "Lobbies")]
#[post("/delete_all")]
async fn delete_all() {
    unsafe {
        if let Some(db) = &mut crate::DB {
            red_lobby::delete_all(db).ok();
        }
    }
}
