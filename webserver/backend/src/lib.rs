use axum::{
    http::{self, Method},
    routing::{get, post},
    Router, Extension,
};
use axum::middleware::from_fn;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

pub mod database;
pub mod handlers;
pub mod middleware;
pub mod validation;
pub mod certificates;

/// Creates the application router with all routes and middleware
pub fn create_app(pool: database::DatabasePool) -> Router {
    // Set up CORS - Allow all origins for development
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::ACCEPT,
            http::header::ORIGIN,
            http::header::HeaderName::from_static("x-api-key"),
        ])
        .allow_origin(Any)
        .allow_credentials(false);

    // Create a router for protected routes (requires API key)
    let protected_routes = Router::new()
        .route("/airquality", post(handlers::create_air_quality_record))
        .route_layer(from_fn(middleware::api_key::api_key_auth));

    // Create a router for public routes (no API key required)
    let public_routes = Router::new()
        .route("/airquality", get(handlers::get_air_quality_record));

    // Combine the routers
    Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool))
}
