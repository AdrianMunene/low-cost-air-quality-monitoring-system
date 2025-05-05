use dotenvy::dotenv;
use std::sync::Arc;
use tracing::{info, error};
use axum_server::tls_rustls::RustlsConfig;

use backend::database::establish_connection_pool;
use backend::create_app;
use backend::middleware::rate_limit::{RateLimiter, rate_limit};
use backend::certificates;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();

    // Set up database connection pool
    let pool = establish_connection_pool();

    // Create rate limiter (10 requests per minute)
    let rate_limiter = Arc::new(RateLimiter::new(10, 60));

    // Create the application with all routes and middleware
    let app = create_app(pool);

    // Add rate limiting middleware
    let app = app
        .layer(axum::middleware::from_fn_with_state(
            rate_limiter.clone(),
            rate_limit,
        ))
        .with_state(rate_limiter);

    // Ensure certificates directory exists
    certificates::ensure_certs_dir();

    // Check if certificates exist, if not, generate them
    if !certificates::certs_exist() {
        info!("Certificates not found. Please run 'cargo run --bin generate_certs' to generate them.");
        error!("HTTPS server cannot start without certificates. Exiting...");
        std::process::exit(1);
    }

    // Get certificate paths
    let (cert_path, key_path) = certificates::get_cert_paths();

    // Configure TLS
    let config = match RustlsConfig::from_pem_file(cert_path, key_path).await {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load TLS configuration: {}", e);
            error!("HTTPS server cannot start. Exiting...");
            std::process::exit(1);
        }
    };

    // Get host and port from environment variables or use defaults
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("{}:{}", host, port);

    // Check if we should use HTTPS or HTTP
    let use_https = std::env::var("USE_HTTPS").unwrap_or_else(|_| "false".to_string()) == "true";

    if use_https {
        info!("HTTPS server with rate limiting listening on https://{}", addr);

        // Start the HTTPS server
        if let Err(e) = axum_server::bind_rustls(addr.parse().unwrap(), config)
            .serve(app.into_make_service())
            .await {
            error!("HTTPS server error: {}", e);
            eprintln!("HTTPS server error: {}", e);
            std::process::exit(1);
        }
    } else {
        info!("HTTP server with rate limiting listening on http://{}", addr);

        // Start the HTTP server
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        if let Err(e) = axum::serve(listener, app).await {
            error!("HTTP server error: {}", e);
            eprintln!("HTTP server error: {}", e);
            std::process::exit(1);
        }
    }
}
