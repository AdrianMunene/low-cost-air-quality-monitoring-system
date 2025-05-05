use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::config::app_config::AppConfig;
use crate::error::api_error::ApiError;

pub async fn api_key_auth(
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Get API key from environment variable
    let app_config = AppConfig::from_env();
    let api_key = app_config.api_key.unwrap_or_default();

    // If no API key is set, skip authentication in development
    if api_key.is_empty() {
        return Ok(next.run(request).await);
    }

    // Check for API key in header
    let auth_header = request
        .headers()
        .get("X-API-Key")
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(key) if key == api_key => {
            // API key is valid, proceed with the request
            Ok(next.run(request).await)
        }
        _ => {
            // API key is invalid or missing
            Err(ApiError::Unauthorized("Invalid or missing API key".to_string()))
        }
    }
}
