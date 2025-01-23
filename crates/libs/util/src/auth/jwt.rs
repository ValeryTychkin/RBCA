use super::TokenType;
use rocket::form::{FromForm, FromFormField};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

pub const OAUTH2_ACCEESS_LIFE_SEC: u32 = 900;
pub const OAUTH2_REFRESH_LIFE_SEC: u32 = 1_296_000;

/// Enum representing the two types of OAuth2 tokens: AccessToken and RefreshToken.
#[derive(Debug, Serialize, Deserialize, JsonSchema, FromFormField)]
#[serde(rename_all = "snake_case")]
pub enum Oauth2TokenType {
    AccessToken,
    RefreshToken,
}

/// Base struct representing the claims within an OAuth2 token.
///
/// # Example
/// ```rust
/// #[derive(Serialize, Deserialize, Debug)]
/// struct UserClaims {
///     id: Uuid,
///     role: String,
///     #[serde(flatten)]
///     oauth2_claims: Oauth2TokenClaims,
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Oauth2TokenClaims {
    pub iat: u64,
    pub nbf: u64,
    pub exp: u64,
    pub jti: Uuid,
    pub sub_jti: Uuid,
    pub oauth_token_type: Oauth2TokenType,
}

pub enum ValidError {
    Invalid,
    TokenNotYetActive,
    TokenExpired,
}

impl Oauth2TokenClaims {
    /// Creates a new pair of access and refresh token claims.
    ///
    /// This method generates new claims for both an access token and a refresh token,
    /// including setting their respective expiration times and unique identifiers.
    pub fn new_claims() -> (Self, Self) {
        let access_id = Uuid::new_v4();
        let refresh_id = Uuid::new_v4();
        let now_unix = OffsetDateTime::now_utc().unix_timestamp() as u64;
        (
            Oauth2TokenClaims {
                iat: now_unix,
                nbf: now_unix,
                sub_jti: refresh_id,
                jti: access_id,
                exp: now_unix + OAUTH2_ACCEESS_LIFE_SEC as u64,
                oauth_token_type: Oauth2TokenType::AccessToken,
            },
            Oauth2TokenClaims {
                iat: now_unix,
                nbf: now_unix,
                sub_jti: access_id,
                jti: refresh_id,
                exp: now_unix + OAUTH2_REFRESH_LIFE_SEC as u64,
                oauth_token_type: Oauth2TokenType::RefreshToken,
            },
        )
    }

    /// Validates whether the token's date range is valid.
    ///
    /// Checks if the token is not yet active (based on the `nbf` field) and if it has not expired
    /// (based on the `exp` field). Returns appropriate error if the token is not valid.
    pub fn validate_date_range(&self) -> Result<(), ValidError> {
        let utc_now = OffsetDateTime::now_utc().unix_timestamp() as u64;
        if self.nbf > utc_now {
            return Err(ValidError::TokenNotYetActive);
        }
        if self.exp < utc_now {
            return Err(ValidError::TokenExpired);
        }
        Ok(())
    }

    /// Returns the life duration of the token in seconds.
    pub fn get_life_sec(&self) -> u64 {
        self.exp - self.nbf
    }

    /// Checks if the token is an access token.
    pub fn is_access(&self) -> bool {
        matches!(self.oauth_token_type, Oauth2TokenType::AccessToken)
    }
}

#[derive(Debug)]
pub enum ExtractError {
    Invalid,
    Empty,
}

/// Extracts the bearer token from an authorization header.
///
/// This function parses the Authorization header to retrieve the token value after the "Bearer " prefix.
/// Returns an error if the header is empty or doesn't start with "Bearer ".
pub fn extract_bearer_token(header: &str) -> Result<String, ExtractError> {
    if !header.is_empty() {
        if let Some(token) = header.strip_prefix("Bearer ") {
            Ok(token.to_owned().to_string())
        } else {
            Err(ExtractError::Invalid)
        }
    } else {
        Err(ExtractError::Empty)
    }
}

/// Struct representing the result of an OAuth2 login.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Oauth2LoginResult {
    pub token_type: TokenType,
    pub expires_in: u64,
    pub access_token: String,
    pub refresh_token: String,
}

/// Struct representing the result of an OAuth2 token introspection.
#[derive(Serialize, Deserialize, Debug, Default, JsonSchema)]
pub struct IntrospectResult {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<u64>,
}

/// Struct representing the input for token introspection.
#[derive(Serialize, Deserialize, Debug, Default, JsonSchema, FromForm)]
pub struct IntrospectInput {
    pub token: String,
    pub token_type_hint: Option<Oauth2TokenType>,
}

impl IntrospectInput {
    /// Determines whether the token is an access token.
    pub fn is_access(&self) -> bool {
        if let Some(token_hint) = &self.token_type_hint {
            return matches!(token_hint, Oauth2TokenType::AccessToken);
        }
        false
    }
}
