use reqwest::Client;
use serde_json::json;
use tokio;
use serde::Deserialize;
use chrono;

#[derive(Debug, Deserialize)]
struct AirQualityData {
    timestamp: String,
    longitude: Option<f64>,
    latitude: Option<f64>,
    location: Option<String>,
    temperature: Option<f64>,
    pressure: Option<f64>,
    humidity: Option<f64>,
    pm1_0: Option<f64>,
    pm2_5: Option<f64>,
    pm10: Option<f64>,
    co2: Option<f64>,
    co: Option<f64>,
    o3: Option<f64>,
}

#[tokio::test]
async fn test_create_air_quality_record() {
    let client = Client::new();

    let url = "http://127.0.0.1:3000/airquality";

    let payload = json!({
        "timestamp": "2025-03-30 12:34:56",
        "longitude": -122.4194,
        "latitude": 37.7749,
        "temperature": 18.5,
        "pressure": 1012.3,
        "humidity": 60.2,
        "pm1_0": 5.1,
        "pm2_5": 10.2,
        "pm10": 20.5,
        "co2": 400.0,
        "co": 0.5,
        "o3": 0.03
    });

    let response = client.post(url).json(&payload).send().await.unwrap();

    assert_eq!(response.status().as_u16(), 200);

    let response_body: serde_json::Value = response.json().await.unwrap();

    /*
    let response_body: serde_json::Value = response.json().await.unwrap_or_else(|err| {
        eprintln!("Error parsing response JSON: {}", err);
        panic!("Failed to parse JSON response.");
    });
    */

    println!("Response Body: {:?}", response_body);

    assert_eq!(response_body["status"], "success");
}

#[tokio::test]
async fn test_location_is_set_by_geocoding() {
    // This test verifies that:
    // 1. We can post data with latitude/longitude but no location
    // 2. When we retrieve the data, the location field is populated by geocoding

    let client = Client::new();
    let base_url = "http://127.0.0.1:3000/airquality";

    // First, create a record with lat/long but no location
    let test_timestamp = format!("2025-03-30 {}", chrono::Utc::now().format("%H:%M:%S"));
    let payload = json!({
        "timestamp": test_timestamp,
        "longitude": -122.4194,  // San Francisco coordinates
        "latitude": 37.7749,
        "temperature": 18.5,
        "pressure": 1012.3,
        "humidity": 60.2,
        "pm1_0": 5.1,
        "pm2_5": 10.2,
        "pm10": 20.5,
        "co2": 400.0,
        "co": 0.5,
        "o3": 0.03
    });

    // Post the data
    let response = client.post(base_url).json(&payload).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 200);

    // Now retrieve all records
    let response = client.get(base_url).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 200);

    let records: Vec<AirQualityData> = response.json().await.unwrap();

    // Find our record by timestamp
    let our_record = records.iter().find(|r| r.timestamp == test_timestamp);
    assert!(our_record.is_some(), "Could not find our test record");

    let record = our_record.unwrap();

    // Verify that location is set, even though we didn't provide it
    assert!(record.location.is_some(), "Location should be set by geocoding");

    // The location should contain some reference to San Francisco or California
    // Note: This test might be flaky if the geocoding service changes or is unavailable
    if let Some(location) = &record.location {
        println!("Geocoded location: {}", location);
        // We don't assert exact content since the geocoding service might change its format
    }
}

#[tokio::test]
async fn test_provided_location_is_ignored() {
    // This test verifies that:
    // 1. If we provide a location in the input, it's ignored
    // 2. The location is set by geocoding based on lat/long instead

    let client = Client::new();
    let base_url = "http://127.0.0.1:3000/airquality";

    // Create a record with lat/long AND a location (which should be ignored)
    let test_timestamp = format!("2025-03-30 {}", chrono::Utc::now().format("%H:%M:%S"));
    let payload = json!({
        "timestamp": test_timestamp,
        "longitude": -122.4194,  // San Francisco coordinates
        "latitude": 37.7749,
        "location": "THIS LOCATION SHOULD BE IGNORED",  // This should be ignored
        "temperature": 18.5,
        "pressure": 1012.3,
        "humidity": 60.2,
        "pm1_0": 5.1,
        "pm2_5": 10.2,
        "pm10": 20.5,
        "co2": 400.0,
        "co": 0.5,
        "o3": 0.03
    });

    // Post the data
    let response = client.post(base_url).json(&payload).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 200);

    // Now retrieve all records
    let response = client.get(base_url).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 200);

    let records: Vec<AirQualityData> = response.json().await.unwrap();

    // Find our record by timestamp
    let our_record = records.iter().find(|r| r.timestamp == test_timestamp);
    assert!(our_record.is_some(), "Could not find our test record");

    let record = our_record.unwrap();

    // Verify that location is set and is NOT the one we provided
    assert!(record.location.is_some(), "Location should be set by geocoding");
    assert_ne!(record.location, Some("THIS LOCATION SHOULD BE IGNORED".to_string()),
               "Location should not be the one we provided");

    // The location should be set by geocoding instead
    if let Some(location) = &record.location {
        println!("Geocoded location (ignoring provided location): {}", location);
    }
}

#[tokio::test]
async fn test_no_location_when_coordinates_missing() {
    // This test verifies that:
    // 1. If we don't provide lat/long, no location is set
    // 2. Even if we provide a location in the input, it's ignored

    let client = Client::new();
    let base_url = "http://127.0.0.1:3000/airquality";

    // Create a record without lat/long but with a location (which should be ignored)
    let test_timestamp = format!("2025-03-30 {}", chrono::Utc::now().format("%H:%M:%S"));
    let payload = json!({
        "timestamp": test_timestamp,
        // No longitude or latitude
        "location": "THIS LOCATION SHOULD BE IGNORED",  // This should be ignored
        "temperature": 18.5,
        "pressure": 1012.3,
        "humidity": 60.2,
        "pm1_0": 5.1,
        "pm2_5": 10.2,
        "pm10": 20.5,
        "co2": 400.0,
        "co": 0.5,
        "o3": 0.03
    });

    // Post the data
    let response = client.post(base_url).json(&payload).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 200);

    // Now retrieve all records
    let response = client.get(base_url).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 200);

    let records: Vec<AirQualityData> = response.json().await.unwrap();

    // Find our record by timestamp
    let our_record = records.iter().find(|r| r.timestamp == test_timestamp);
    assert!(our_record.is_some(), "Could not find our test record");

    let record = our_record.unwrap();

    // Verify that location is NOT set since we didn't provide coordinates
    assert!(record.location.is_none(), "Location should not be set when coordinates are missing");

    // Also verify that the provided location was ignored
    assert_ne!(record.location, Some("THIS LOCATION SHOULD BE IGNORED".to_string()),
               "Provided location should be ignored");
}