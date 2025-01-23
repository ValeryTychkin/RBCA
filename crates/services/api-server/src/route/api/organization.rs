use crate::{
    merdge_mulit_routes,
    query::organization as org_query,
    schema::{auth::SelfUserTokenClaims, organization as org_schema},
    usecase::organization as org_usecase,
};
use rocket::{http::Status, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings};
use uuid::Uuid;

#[openapi(tag = "Organization")]
#[get("/?<req_query..>")]
pub async fn get_miltiple(
    req_query: org_query::Organization,
    _user_claims: SelfUserTokenClaims,
) -> Json<org_schema::OrganizationList> {
    Json(org_usecase::get_all(req_query).await)
}

#[openapi(tag = "Organization")]
#[get("/<id>")]
pub async fn get_by_id(
    id: Uuid,
    _user_claims: SelfUserTokenClaims,
) -> (Status, Option<Json<org_schema::Organization>>) {
    let res = org_usecase::get_by_id(id).await;
    match res {
        Some(v) => (Status::Ok, Some(Json(v))),
        None => (Status::NotFound, None),
    }
}

#[openapi(tag = "Organization")]
#[post("/", data = "<org>")]
pub async fn create(
    org: Json<org_schema::OrganizationCreate>,
    _user_claims: SelfUserTokenClaims,
) -> (Status, Json<org_schema::Organization>) {
    (Status::Created, Json(org_usecase::create_new(org.0).await))
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    merdge_mulit_routes![settings, [get_miltiple, create, get_by_id]]
}
