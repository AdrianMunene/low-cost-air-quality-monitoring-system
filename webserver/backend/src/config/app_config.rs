use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub api_key: Option<String>,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let api_key = env::var("API_KEY").ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env");

        Self {
            api_key,
            database_url,
        }
    }
}
