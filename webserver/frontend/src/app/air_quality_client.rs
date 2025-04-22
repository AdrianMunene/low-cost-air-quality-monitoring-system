use reqwest::Client;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::cell::RefCell;

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

// Global cache for air quality data
thread_local! {
    static LAST_FETCH_TIME: RefCell<Option<Instant>> = RefCell::new(None);
    static CACHED_DATA: RefCell<Option<Vec<AirQualityData>>> = RefCell::new(None);
    static IS_FETCHING: RefCell<bool> = RefCell::new(false);
}

// Minimum time between API requests (in milliseconds)
const THROTTLE_MS: u64 = 5000; // 5 seconds

// Function to get air quality data with throttling
pub async fn get_air_quality_data() -> Result<Vec<AirQualityData>, String> {
    // Check if we should make a new request
    let should_fetch = LAST_FETCH_TIME.with(|last_time| {
        let mut last = last_time.borrow_mut();
        let now = Instant::now();

        match *last {
            None => {
                // First request, update time and fetch
                *last = Some(now);
                true
            },
            Some(time) => {
                // Check if enough time has passed
                if now.duration_since(time) >= Duration::from_millis(THROTTLE_MS) {
                    *last = Some(now);
                    true
                } else {
                    // Not enough time has passed, use cached data
                    false
                }
            }
        }
    });

    // Check if we're already fetching
    let already_fetching = IS_FETCHING.with(|flag| *flag.borrow());

    if should_fetch && !already_fetching {
        // Set fetching flag
        IS_FETCHING.with(|flag| *flag.borrow_mut() = true);

        // Make the API request
        let client = Client::new();
        let result = match client.get("http://127.0.0.1:3000/airquality").send().await {
            Ok(response) => match response.json::<Vec<AirQualityData>>().await {
                Ok(data) => {
                    // Update cache
                    CACHED_DATA.with(|cache| {
                        *cache.borrow_mut() = Some(data.clone());
                    });
                    Ok(data)
                },
                Err(e) => Err(format!("Failed to parse response: {:?}", e))
            },
            Err(e) => Err(format!("Error fetching data: {:?}", e)),
        };

        // Reset fetching flag
        IS_FETCHING.with(|flag| *flag.borrow_mut() = false);

        return result;
    } else {
        // Return cached data if available
        return CACHED_DATA.with(|cache| {
            match &*cache.borrow() {
                Some(data) => Ok(data.clone()),
                None => {
                    // If no cached data, return an empty result
                    // The next call will fetch the data
                    Ok(Vec::new())
                }
            }
        });
    }
}