use crate::{
    merdge_mulit_routes,
    query::user as user_query,
    schema::{auth::SelfUserTokenClaims, user as user_schema},
    usecase::user as user_usecase,
};
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings};

#[openapi(tag = "User")]
#[get("/?<req_query..>")]
pub async fn get_multiple(
    req_query: user_query::User,
    _user_claims: SelfUserTokenClaims,
) -> Json<user_schema::UserList> {
    Json(user_usecase::get_all(req_query).await)
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    merdge_mulit_routes![settings, [get_multiple]]
}
