use axum::{
    http::{self, Method},
    middleware::from_fn,
    routing::{get, post},
    Router, Extension,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

use crate::database::connection::DatabasePool;
use crate::middleware::api_key::api_key_auth;
use crate::api::air_quality::{create_air_quality_record, get_air_quality_record};

/// Creates the application router with all routes and middleware
pub fn create_router(pool: DatabasePool) -> Router {
    // Set up CORS - Allow all origins for development with more permissive settings
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::HEAD])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::ACCEPT,
            http::header::ORIGIN,
            http::header::AUTHORIZATION,
            http::header::HeaderName::from_static("x-api-key"),
        ])
        .allow_origin(Any)
        .allow_credentials(false)
        .expose_headers([
            http::header::CONTENT_TYPE,
            http::header::CONTENT_LENGTH,
        ])
        .max_age(std::time::Duration::from_secs(86400)); // 24 hours

    // Create a router for protected routes (requires API key)
    let protected_routes = Router::new()
        .route("/airquality", post(create_air_quality_record))
        .route_layer(from_fn(api_key_auth));

    // Create a router for public routes (no API key required)
    let public_routes = Router::new()
        .route("/airquality", get(get_air_quality_record));

    // Combine the routers
    Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool))
}
