use crate::{merdge_mulit_routes, schema::base::Result};
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings};
use tokio::time::{sleep, Duration};

#[openapi(tag = "Delays")]
#[get("/<seconds>")]
pub async fn delay_sec(seconds: u64) -> Json<Result> {
    sleep(Duration::from_secs(seconds)).await;
    Json(Result {
        data: format!("Wake up after {seconds} sec."),
    })
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    merdge_mulit_routes![settings, [delay_sec]]
}
