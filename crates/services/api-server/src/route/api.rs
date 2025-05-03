mod application;
mod auth;
mod user;
mod user_staff;

use rocket_okapi::{
    get_nested_endpoints_and_docs, okapi::openapi3::OpenApi, settings::OpenApiSettings,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/user" => user::get_routes_and_docs(settings),
        "/auth" => auth::get_routes_and_docs(settings),
        "/user-staff" => user_staff::get_routes_and_docs(settings),
        "/application" => application::get_routes_and_docs(settings)
    }
}
