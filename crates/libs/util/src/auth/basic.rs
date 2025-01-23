use base64::prelude::*;

/// Extracts and decodes the basic authentication credentials from an Authorization header.
///
/// This function expects the header to have the "Basic" prefix followed by a Base64-encoded string.
/// It decodes the credentials and returns a tuple containing the username and password.
///
/// # Arguments
/// - `header`: The `Authorization` header containing the Base64-encoded credentials.
///
/// # Returns
/// A `Result` containing:
/// - `Ok((String, String))`: The decoded username and password.
/// - `Err(&'static str)`: An error message if the header format or decoding fails.
///
/// # Errors
/// - `"Invalid Base64 encoding"` if the Base64 decoding fails.
/// - `"Invalid UTF-8 in credentials"` if the decoded string is not valid UTF-8.
/// - `"Invalid format for Basic credentials"` if the decoded string doesn't contain a colon separating the username and password.
/// - `"Invalid Authorization header format"` if the header does not start with "Basic ".
pub fn extract_basic_credentials(header: &str) -> Result<(String, String), &'static str> {
    if let Some(encoded) = header.strip_prefix("Basic ") {
        let decoded = BASE64_STANDARD
            .decode(encoded)
            .map_err(|_| "Invalid Base64 encoding")?;
        let decoded_str = String::from_utf8(decoded).map_err(|_| "Invalid UTF-8 in credentials")?;

        if let Some((username, password)) = decoded_str.split_once(':') {
            Ok((username.to_string(), password.to_string()))
        } else {
            Err("Invalid format for Basic credentials")
        }
    } else {
        Err("Invalid Authorization header format")
    }
}
