use axum::{ http::Method, routing::{ get, post }, Router, Extension };
use dotenvy::dotenv;
use tower_http::cors::{ CorsLayer, Any };

use database::establish_connection_pool;
use handlers::{create_air_quality_record, get_air_quality_record};

mod database;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = establish_connection_pool();

    let cors = CorsLayer::new().allow_methods(vec![Method::GET, Method::POST]).allow_origin(Any);

    let app = Router::new()
    .route("/airquality", get(get_air_quality_record))
    .route("/airquality", post(create_air_quality_record))
    .layer(cors)
    .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on 127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}