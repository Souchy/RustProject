use coral_commons::{protos::models::{Match, MatchState}, red::red_match};
use rocket::{get, post, serde::json::Json};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get, get_all, delete_all]
}

#[openapi(tag = "Matches")]
#[get("/<id>")]
async fn get(id: String) -> Json<Option<Match>> {
    unsafe {
        if let Some(db) = &mut crate::DB {
            let opt_match = red_match::get(db, &id).ok();
            return Json(opt_match);
        }
    }
    return Json(None);
}

#[openapi(tag = "Matches")]
#[get("/all/<state>")]
async fn get_all(state: i32) -> Json<Vec<Match>> {
    unsafe {
        if let Some(db) = &mut crate::DB {
            let opt_index = red_match::get_index(db).ok();
            let mut matches: Vec<Match> = Vec::new();
            if let Some(index) = opt_index {
                for id in index.iter() {
                    let opt_match = red_match::get(db, id).ok();
                    if let Some(r#match) = opt_match {
                        if r#match.state == state {
                            matches.push(r#match);
                        }
                    }
                }
                return Json(matches);
            }
        }
    }
    return Json(vec![]);
}

#[openapi(tag = "Matches")]
#[post("/delete_all")]
async fn delete_all() {
    unsafe {
        if let Some(db) = &mut crate::DB {
            red_match::delete_all(db).ok();
        }
    }
}
