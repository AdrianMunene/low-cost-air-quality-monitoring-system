use reqwest::Client;
use serde_json::json;
use tokio;

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