use std::sync::Arc;
use axum_server::tls_rustls::RustlsConfig;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

use backend::config::app_config::AppConfig;
use backend::config::server_config::ServerConfig;
use backend::database::connection::establish_connection_pool;
use backend::api::router::create_router;
use backend::middleware::rate_limit::{RateLimiter, rate_limit};
use backend::security::certificates::{ensure_certs_dir, certs_exist, get_cert_paths};

#[tokio::main]
async fn main() {
    // Initialize configuration
    let _app_config = AppConfig::from_env(); // Available for future use
    let server_config = ServerConfig::from_env();

    // Initialize tracing
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug,axum=debug"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    if server_config.use_https {
        info!("Starting secure HTTPS server with rate limiting...");
    } else {
        info!("Starting HTTP server with rate limiting...");
    }

    // Set up database connection pool
    let pool = establish_connection_pool();

    // Create rate limiter from server config
    info!("Rate limiting set to {} requests per minute", server_config.rate_limit_per_minute);
    let rate_limiter = Arc::new(RateLimiter::new(server_config.rate_limit_per_minute, 60));

    // Create the application with all routes and middleware
    let app = create_router(pool);

    // Add rate limiting middleware with state
    let app = app
        .layer(axum::middleware::from_fn_with_state(
            rate_limiter.clone(),
            rate_limit,
        ))
        .with_state(());

    // Ensure certificates directory exists
    ensure_certs_dir();

    // Start the server based on configuration
    if server_config.use_https {
        // Check if certificates exist, if not, generate them
        if !certs_exist() {
            info!("Certificates not found. Please run 'cargo run --bin generate_certs' to generate them.");
            error!("HTTPS server cannot start without certificates. Exiting...");
            std::process::exit(1);
        }

        // Get certificate paths
        let (cert_path, key_path) = get_cert_paths();

        // Configure TLS
        let config = match RustlsConfig::from_pem_file(cert_path, key_path).await {
            Ok(config) => config,
            Err(e) => {
                error!("Failed to load TLS configuration: {}", e);
                error!("HTTPS server cannot start. Exiting...");
                std::process::exit(1);
            }
        };

        info!("HTTPS server listening on https://{}:{}", server_config.host, server_config.port);

        // Start the HTTPS server with ConnectInfo
        if let Err(e) = axum_server::bind_rustls(server_config.socket_addr(), config)
            .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
            .await {
            error!("HTTPS server error: {}", e);
            eprintln!("HTTPS server error: {}", e);
            std::process::exit(1);
        }
    } else {
        info!("HTTP server listening on http://{}:{}", server_config.host, server_config.port);

        // Start the HTTP server with ConnectInfo
        let listener = tokio::net::TcpListener::bind(server_config.socket_addr()).await.unwrap();
        if let Err(e) = axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
        ).await {
            error!("HTTP server error: {}", e);
            eprintln!("HTTP server error: {}", e);
            std::process::exit(1);
        }
    }
}