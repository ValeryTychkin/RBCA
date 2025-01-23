mod api;
mod core;

use rocket::{Build, Rocket};

use rocket_okapi::{
    get_nested_endpoints_and_docs, mount_endpoints_and_merged_docs,
    okapi::openapi3::OpenApi,
    settings::OpenApiSettings,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/api" => api::get_routes_and_docs(settings),
        "/" => core::get_routes_and_docs(settings),
    }
}

pub fn init_routes(build: Rocket<Build>) -> Rocket<Build> {
    let mut build = build.mount(
        "/swagger",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    );

    let openapi_settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        build, "/".to_owned(), openapi_settings,
        "/" => get_routes_and_docs(&openapi_settings),
    };

    build
}
