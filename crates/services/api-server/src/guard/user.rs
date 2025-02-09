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

use super::GuardError;

#[derive(Debug)]
pub struct User {
    pub claims: SelfUserTokenClaims,
}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = GuardError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(header_value) = request.headers().get_one("authorization") {
            if let Ok(token_str) = auth_jwt::extract_bearer_token(header_value) {
                // Parse the token into a SelfUserTokenClaims object
                if let Ok(user_claims) = SelfUserTokenClaims::from_jwt(&token_str) {
                    // Ensure the OAuth2 claims are valid (access type)
                    if user_claims.oauth2_claims.is_access() {
                        if let Ok(_) = user_claims.oauth2_claims.validate_date_range() {
                            if auth_usecase::token_is_exist(&user_claims).await {
                                return Outcome::Success(Self {
                                    claims: user_claims,
                                });
                            }
                            return Outcome::Error((
                                Status::Unauthorized,
                                GuardError::MissingToken,
                            ));
                        }
                    }
                }
            }
            return Outcome::Error((Status::Unauthorized, GuardError::WrongToken));
        }
        Outcome::Error((Status::Unauthorized, GuardError::MissingAuthToken))
    }
}

// Implementing OpenApiFromRequest to document the API security requirements in OpenAPI format
impl<'a> OpenApiFromRequest<'a> for User {
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
