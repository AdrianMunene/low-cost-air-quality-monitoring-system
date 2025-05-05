use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn api_key_auth(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get API key from environment variable
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "".to_string());
    
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
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
