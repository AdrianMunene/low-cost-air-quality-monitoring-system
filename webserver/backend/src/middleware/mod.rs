use axum::{
    extract::{Request, ConnectInfo},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::config::AppConfig;
use crate::error::ApiError;

// API Key Authentication Middleware
pub mod api_key {
    use super::*;

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
}

// Rate Limiting Middleware
pub mod rate_limit {
    use super::*;

    // Simple in-memory rate limiter
    pub struct RateLimiter {
        // Maps IP addresses to (request count, last request time)
        requests: Mutex<HashMap<String, (u32, Instant)>>,
        max_requests: u32,
        window: Duration,
    }

    impl RateLimiter {
        pub fn new(max_requests: u32, window_seconds: u64) -> Self {
            Self {
                requests: Mutex::new(HashMap::new()),
                max_requests,
                window: Duration::from_secs(window_seconds),
            }
        }

        pub fn is_rate_limited(&self, ip: &str) -> bool {
            let mut requests = self.requests.lock().unwrap();
            let now = Instant::now();

            match requests.get(ip).cloned() {
                Some((count, time)) => {
                    if now.duration_since(time) > self.window {
                        // Reset if window has passed
                        requests.insert(ip.to_string(), (1, now));
                        false
                    } else if count >= self.max_requests {
                        // Rate limited
                        true
                    } else {
                        // Increment count
                        requests.insert(ip.to_string(), (count + 1, time));
                        false
                    }
                },
                None => {
                    // First request from this IP
                    requests.insert(ip.to_string(), (1, now));
                    false
                }
            }
        }
    }

    pub async fn rate_limit(
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        limiter: axum::extract::State<Arc<RateLimiter>>,
        request: Request,
        next: Next,
    ) -> Result<Response, ApiError> {
        let ip = addr.ip().to_string();

        // Exempt localhost and local network connections from rate limiting
        // This allows the frontend to make unlimited requests
        if ip == "127.0.0.1" || ip == "::1" || ip.starts_with("192.168.") || ip.starts_with("10.") {
            return Ok(next.run(request).await);
        }

        if limiter.is_rate_limited(&ip) {
            return Err(ApiError::TooManyRequests);
        }

        Ok(next.run(request).await)
    }
}
