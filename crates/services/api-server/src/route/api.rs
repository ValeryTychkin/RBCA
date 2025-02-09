mod auth;
mod user;

use rocket_okapi::{
    get_nested_endpoints_and_docs, okapi::openapi3::OpenApi, settings::OpenApiSettings,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/user" => user::get_routes_and_docs(settings),
        "/auth" => auth::get_routes_and_docs(settings),
    }
}
