use crate::{
    guard::user as user_guard,
    merdge_mulit_routes,
    schema::{
        auth::{Login, Register},
        base::ErrorResult,
        user::User,
    },
    usecase::auth::{self as auth_usecase, ErrorLogin},
};
use rocket::{form::Form, http::Status, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings};
use util_lib::auth::jwt::{IntrospectInput, IntrospectResult, Oauth2LoginResult};

#[openapi(tag = "Auth")]
#[post("/register", data = "<user_reg>")]
pub async fn register(user_reg: Json<Register>) -> (Status, Result<Json<User>, Json<ErrorResult>>) {
    match auth_usecase::registration(user_reg.0).await {
        Ok(v) => (Status::Created, Ok(Json(v))),
        Err(v) => (Status::Conflict, Err(Json(v))),
    }
}

#[openapi(tag = "Auth")]
#[post("/login", data = "<user_login>")]
pub async fn login(
    user_login: Json<Login>,
) -> (Status, Result<Json<Oauth2LoginResult>, Json<ErrorResult>>) {
    match auth_usecase::login(user_login.0).await {
        Ok(v) => (Status::Ok, Ok(Json(v))),
        Err(v) => match v {
            ErrorLogin::Email => (
                Status::BadRequest,
                Err(Json(ErrorResult {
                    err_msg: "email doesn't exist".to_string(),
                    err_detail: None,
                })),
            ),
            ErrorLogin::Password => (
                Status::BadRequest,
                Err(Json(ErrorResult {
                    err_msg: "incorrect password".to_string(),
                    err_detail: None,
                })),
            ),
        },
    }
}

#[openapi(tag = "Auth")]
#[post("/logout")]
pub async fn logout(user: user_guard::User) -> Status {
    auth_usecase::logout(user.claims).await;
    Status::NoContent
}

#[openapi(tag = "Auth")]
#[post("/introspect", data = "<token_intro>")]
pub async fn introspect(token_intro: Form<IntrospectInput>) -> Json<IntrospectResult> {
    Json(auth_usecase::introspect(token_intro.into_inner()).await)
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    merdge_mulit_routes![settings, [register, login, logout, introspect]]
}
