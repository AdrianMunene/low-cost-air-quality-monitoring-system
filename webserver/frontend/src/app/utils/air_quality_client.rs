use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
pub struct AirQualityData {
    pub timestamp: String,
    pub temperature: Option<f64>,
    pub pressure: Option<f64>,
    pub humidity: Option<f64>,
    pub pm1_0: Option<f64>,
    pub pm2_5: Option<f64>,
    pub pm10: Option<f64>,
    pub co2: Option<f64>,
    pub co: Option<f64>,
    pub o3: Option<f64>,
}

// Try both HTTPS and HTTP endpoints
pub async fn get_air_quality_data() -> Result<Vec<AirQualityData>, String> {
    // First try HTTPS
    match try_get_data(true).await {
        Ok(data) => Ok(data),
        Err(https_err) => {
            log::warn!("HTTPS request failed: {}. Trying HTTP...", https_err);
            // If HTTPS fails, try HTTP
            try_get_data(false).await
        }
    }
}

async fn try_get_data(use_https: bool) -> Result<Vec<AirQualityData>, String> {
    // Build standard client - reqwest in WASM automatically accepts invalid certs
    let client = Client::new();

    // Determine protocol
    let protocol = if use_https { "https" } else { "http" };
    let url = format!("{}://127.0.0.1:3001/airquality", protocol);

    log::info!("Attempting to fetch data from: {}", url);

    match client.get(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send().await {
        Ok(response) => {
            if !response.status().is_success() {
                return Err(format!("Server returned error status: {}", response.status()));
            }

            match response.json::<Vec<AirQualityData>>().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse response: {:?}", e))
            }
        }
        Err(e) => Err(format!("Error fetching data from {}: {:?}", url, e)),
    }
}