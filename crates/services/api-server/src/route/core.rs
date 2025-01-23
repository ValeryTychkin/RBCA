use crate::merdge_mulit_routes;

use rocket::http::Status;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings};

#[openapi(tag = "Core")]
#[get("/healthz")]
fn health() -> Status {
    Status::Ok
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    merdge_mulit_routes![settings, [health]]
}
