use serde::{ Serialize, Deserialize };
use axum::{ Extension, Json };
use chrono::NaiveDateTime;
use serde_json::json;
use diesel::prelude::*;
use database::models::{AirQualityData, NewAirQualityData};
use database::schema::air_quality_data::dsl::air_quality_data;
use crate::database::DatabasePool;
use crate::geocoding::reverse_geocode;

/// Helper function to get location from coordinates
/// This is extracted to make it easier to test
pub async fn get_location_from_coordinates(latitude: Option<f64>, longitude: Option<f64>) -> Option<String> {
    match (latitude, longitude) {
        (Some(lat), Some(lon)) => {
            match reverse_geocode(lat, lon).await {
                Ok(loc) => Some(loc),
                Err(e) => {
                    eprintln!("Geocoding error: {}", e);
                    None // Don't use any fallback, just store None if geocoding fails
                }
            }
        },
        _ => None // No location if coordinates are not available
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AirQualityInputOutput {
    pub timestamp: String,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    // Location is determined by the backend using geocoding based on latitude/longitude
    // It should not be provided in the input, but will be included in the output
    pub location: Option<String>,
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

pub async fn create_air_quality_record(
    Extension(pool): Extension<DatabasePool>,
    Json(input): Json<AirQualityInputOutput>,
) -> Result<Json<serde_json::Value>, String> {

    let mut conn = pool.get().map_err(|e| e.to_string())?;

    let timestamp = NaiveDateTime::parse_from_str(&input.timestamp, "%Y-%m-%d %H:%M:%S")
    .map_err(|e| format!("Invalid timestamp: {}", e))?;

    // Perform reverse geocoding if latitude and longitude are provided
    // Note: We always use geocoding for location when coordinates are available,
    // and we ignore any location that might have been provided in the input
    let location = get_location_from_coordinates(input.latitude, input.longitude).await;

    let new_record = NewAirQualityData {
        timestamp,
        longitude: input.longitude,
        latitude: input.latitude,
        location,
        temperature: input.temperature,
        pressure: input.pressure,
        humidity: input.humidity,
        pm1_0: input.pm1_0,
        pm2_5: input.pm2_5,
        pm10: input.pm10,
        co2: input.co2,
        co: input.co,
        o3: input.o3,
    };

    diesel::insert_into(air_quality_data)
    .values(&new_record)
    .execute(&mut conn)
    .map_err(|e| e.to_string())?;

    Ok(Json(json!({ "status": "success" })))
}

pub async fn get_air_quality_record(
    Extension(pool): Extension<DatabasePool>
) -> Result<Json<Vec<AirQualityInputOutput>>, String> {

    let mut conn = pool.get().map_err(|e| e.to_string())?;

    let records = air_quality_data.load::<AirQualityData>(&mut conn).map_err(|e| e.to_string())?;

    let output: Vec<AirQualityInputOutput> = records.into_iter().map(|record| {
        AirQualityInputOutput {
            timestamp: record.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            longitude: record.longitude,
            latitude: record.latitude,
            location: record.location,
            temperature: record.temperature,
            pressure: record.pressure,
            humidity: record.humidity,
            pm1_0: record.pm1_0,
            pm2_5: record.pm2_5,
            pm10: record.pm10,
            co2: record.co2,
            co: record.co,
            o3: record.o3,
        }
    }).collect();

    Ok(Json(output))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use std::sync::Once;

    // Initialize logging for tests
    static INIT: Once = Once::new();
    fn init() {
        INIT.call_once(|| {
            // Initialize logging if needed
        });
    }

    #[tokio::test]
    async fn test_get_location_from_coordinates_with_valid_coordinates() {
        init();

        // This test is commented out to avoid making actual API calls during tests
        // Uncomment to test manually
        /*
        let latitude = Some(37.7749);
        let longitude = Some(-122.4194);

        let location = get_location_from_coordinates(latitude, longitude).await;

        assert!(location.is_some(), "Location should be returned for valid coordinates");
        println!("Geocoded location: {:?}", location);
        */
    }

    #[tokio::test]
    async fn test_get_location_from_coordinates_with_missing_coordinates() {
        init();

        // Test with missing latitude
        let location1 = get_location_from_coordinates(None, Some(-122.4194)).await;
        assert!(location1.is_none(), "Location should be None when latitude is missing");

        // Test with missing longitude
        let location2 = get_location_from_coordinates(Some(37.7749), None).await;
        assert!(location2.is_none(), "Location should be None when longitude is missing");

        // Test with both missing
        let location3 = get_location_from_coordinates(None, None).await;
        assert!(location3.is_none(), "Location should be None when both coordinates are missing");
    }
}