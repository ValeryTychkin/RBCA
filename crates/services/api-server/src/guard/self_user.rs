use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use util_lib::auth::jwt as auth_jwt;

use crate::{schema::auth::SelfUserTokenClaims, usecase::auth as auth_usecase};

#[derive(Debug)]
pub enum UserError {
    MissingAuthToken,
    WrongToken,
    MissingToken,
}

#[async_trait]
impl<'r> FromRequest<'r> for SelfUserTokenClaims {
    type Error = UserError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(header_value) = request.headers().get_one("authorization") {
            if let Ok(token_str) = auth_jwt::extract_bearer_token(header_value) {
                // Parse the token into a SelfUserTokenClaims object
                if let Ok(user) = Self::from_jwt(&token_str) {
                    // Ensure the OAuth2 claims are valid (access type)
                    if user.oauth2_claims.is_access() {
                        if let Ok(_) = user.oauth2_claims.validate_date_range() {
                            if auth_usecase::token_is_exist(&user).await {
                                return Outcome::Success(user);
                            }
                            return Outcome::Error((Status::Unauthorized, UserError::MissingToken));
                        }
                    }
                }
            }
            return Outcome::Error((Status::Unauthorized, UserError::WrongToken));
        }
        Outcome::Error((Status::Unauthorized, UserError::MissingAuthToken))
    }
}

// Implementing OpenApiFromRequest to document the API security requirements in OpenAPI format
impl<'a> OpenApiFromRequest<'a> for SelfUserTokenClaims {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requires an Bearer token to access".to_owned()),
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("Authorization".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "Authorization".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
