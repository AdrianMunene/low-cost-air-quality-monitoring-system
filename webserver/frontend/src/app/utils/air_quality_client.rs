use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
pub struct AirQualityData {
    pub timestamp: String,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub location: Option<String>,
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

pub async fn get_air_quality_data() -> Result<Vec<AirQualityData>, String> {
    let client = Client::new();

    //match client.get("https://airqualitymonitoring.cc/airquality").send().await {
    match client.get("http://127.0.0.1:3000/airquality").send().await {
        Ok(response) => match response.json::<Vec<AirQualityData>>().await {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Failed to parse response: {:?}", e))

        }
        Err(e) => Err(format!("Error fetching data: {:?}", e)),
    }

}