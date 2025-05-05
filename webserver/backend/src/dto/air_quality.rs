use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AirQualityInputOutput {
    pub timestamp: String,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub temperature: Option<f64>,
    pub pressure: Option<f64>,
    pub humidity: Option<f64>,
    pub pm1_0: Option<f64>,
    pub pm2_5: Option<f64>,
    pub pm10: Option<f64>,
    pub co2: Option<f64>,
    pub co: Option<f64>,
    pub o3: Option<f64>
}
