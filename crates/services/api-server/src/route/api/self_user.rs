use rocket::{http::Status, serde::json::Json};
use rocket_okapi::{
    get_nested_endpoints_and_docs, okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings,
};

use crate::{
    guard::user as user_guard, merdge_mulit_routes, schema::user as user_schema,
    usecase::user as user_usecase,
};

// TODO: Add captcha chellenge for update password
#[openapi(tag = "Self User")]
#[post("/update-password", data = "<passwords>")]
pub async fn update_password(
    user: user_guard::User,
    passwords: Json<user_schema::UpdateUserPassword>,
) -> Status {
    match user_usecase::update_password(user.claims.id.to_owned(), &passwords).await {
        Ok(_) => Status::Ok,
        Err(e) => match e {
            user_usecase::ErrorUpdatePassword::UserNotFound => Status::BadRequest,
            user_usecase::ErrorUpdatePassword::WrongOldPassword => Status::Conflict,
        },
    }
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/" => merdge_mulit_routes![settings, [update_password]],
    }
}
