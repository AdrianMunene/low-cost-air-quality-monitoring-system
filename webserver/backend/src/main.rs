use axum::{ routing::post, Router, Extension };
use dotenvy::dotenv;

use database::establish_connection_pool;
use handlers::create_air_quality_record;

mod database;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = establish_connection_pool();

    let app = Router::new().route("/airquality", post(create_air_quality_record)).layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on 127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}