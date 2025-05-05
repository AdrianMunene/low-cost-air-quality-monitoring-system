use reqwest::{Client, ClientBuilder};
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
    // Try HTTPS first as it's the preferred method
    match try_get_data(true).await {
        Ok(data) => Ok(data),
        Err(https_err) => {
            log::warn!("HTTPS request failed: {}. Trying HTTP as fallback...", https_err);
            // If HTTPS fails, try HTTP as fallback
            try_get_data(false).await
        }
    }
}

async fn try_get_data(use_https: bool) -> Result<Vec<AirQualityData>, String> {
    // Build client with more permissive settings for development
    let client = ClientBuilder::new()
        .build()
        .unwrap_or_else(|_| Client::new());

    // Determine protocol and port
    let protocol = if use_https { "https" } else { "http" };

    // Try different endpoints - prioritize localhost for better name resolution
    let endpoints = [
        format!("{}://localhost:3000/airquality", protocol),
        format!("{}://127.0.0.1:3000/airquality", protocol),
    ];

    for url in endpoints.iter() {
        log::info!("Attempting to fetch data from: {}", url);

        match client.get(url)
            .header("Accept", "application/json")
            .send().await {
            Ok(response) => {
                log::info!("Received response with status: {}", response.status());

                if !response.status().is_success() {
                    log::warn!("Server returned error status: {}", response.status());
                    continue;
                }

                match response.json::<Vec<AirQualityData>>().await {
                    Ok(data) => {
                        log::info!("Successfully parsed response data with {} records", data.len());
                        return Ok(data);
                    },
                    Err(e) => {
                        log::error!("Failed to parse response: {:?}", e);
                        continue;
                    }
                }
            }
            Err(e) => {
                log::error!("Error fetching data from {}: {:?}", url, e);
                continue;
            }
        }
    }

    Err("Failed to fetch data from any endpoint".to_string())
}