use axum::{
    extract::{Request, ConnectInfo},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::error::ApiError;

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

    if limiter.is_rate_limited(&ip) {
        return Err(ApiError::TooManyRequests);
    }

    Ok(next.run(request).await)
}
