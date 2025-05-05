use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;

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

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub use_https: bool,
    pub rate_limit_per_minute: u32,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");
        let use_https = env::var("USE_HTTPS").unwrap_or_else(|_| "true".to_string()) == "true";
        let rate_limit_per_minute = env::var("RATE_LIMIT_PER_MINUTE")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<u32>()
            .unwrap_or(60);

        Self {
            host,
            port,
            use_https,
            rate_limit_per_minute,
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Failed to parse socket address")
    }
}
