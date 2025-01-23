use crate::settings::SETTINGS;
pub use jsonwebtoken::errors::Result as JWTResult;
use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{de::DeserializeOwned, Serialize};

/// Encodes the provided claims into a JWT token.
///
/// This function is used to create a JWT token using the secret key from settings. It takes a struct that is serializable to JSON,
/// and encodes it into a string using the HMAC algorithm with the secret key from the settings.
///
/// # Parameters
/// - `claims`: A serializable structure that will be used as the payload (claims) of the token.
///
/// # Return Value
/// Returns a result with a string containing the JWT token. If an error occurs, it returns `Err`.
///
/// # Example Usage:
/// ```rust
/// let claims = MyClaims { user_id: 123 };
/// let token = encode(&claims).unwrap();
/// ```
pub fn encode<ClaimsType: Serialize>(claims: &ClaimsType) -> JWTResult<String> {
    jwt_encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(SETTINGS.jwt.secret.as_ref()),
    )
}

/// Decodes the provided JWT token into claims.
///
/// This function is used to decode a JWT token into the claims of the specified type using the secret key from settings.
/// It validates the token using the default validation settings.
///
/// # Parameters
/// - `token`: The JWT token to decode.
///
/// # Return Value
/// Returns a result containing the decoded token data, which includes the claims. If an error occurs, it returns `Err`.
///
/// # Example Usage:
/// ```rust
/// let token = "jwt_token_here".to_string();
/// let decoded = decode::<MyClaims>(&token).unwrap();
/// ```
pub fn decode<Claims: DeserializeOwned>(token: &String) -> JWTResult<TokenData<Claims>> {
    jwt_decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SETTINGS.jwt.secret.as_ref()),
        &Validation::default(),
    )
}
