pub mod api_lobbies;
pub mod api_matches;
pub mod api_players;

use rocket::{Ignite, Rocket};
use rocket_okapi::{
    get_nested_endpoints_and_docs, mount_endpoints_and_merged_docs,
    okapi::openapi3::OpenApi,
    rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig},
    settings::{OpenApiSettings, UrlObject},
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use std::{env, net::IpAddr, str::FromStr};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/lobbies" => api_lobbies::get_routes_and_docs(settings),
        "/matches" => api_matches::get_routes_and_docs(settings),
        "/players" => api_players::get_routes_and_docs(settings),
    }
}

/**
 * Start api server
 */
pub async fn rocket_launch() -> Result<Rocket<Ignite>, rocket::error::Error> {
    let port = env::var("API_PORT")
        .unwrap_or(9000.to_string())
        .parse::<u16>()
        .unwrap();
    let add = env::var("API_ADDR").unwrap_or("127.0.0.1".to_string());

    println!("Starting Rocket on {}:{}", add, port);

    let address = IpAddr::from_str(&add).unwrap();

    let config = rocket::config::Config {
        port,
        address,
        ..Default::default()
    };

    let mut building_rocket = rocket::build()
        .configure(config)
        // .mount("/", routes)
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
        );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/" => get_routes_and_docs(&openapi_settings),
    };

    building_rocket.launch().await
}
