pub mod api_player;
pub mod api_queue;

use rocket::{Ignite, Rocket};
use rocket_okapi::{
    rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig},
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

/**
 * Start api server
 */
pub async fn rocket_launch() -> Result<Rocket<Ignite>, rocket::error::Error> {
    let config = rocket::Config {
        port: 7777,
        ..rocket::Config::debug_default()
    };

    rocket::build()
        .configure(&config)
        .mount("/queue", api_queue::get_routes())
        .mount("/player", api_player::get_routes())
        .mount(
            "/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await
}
