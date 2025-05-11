use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct NominatimResponse {
    display_name: String,
    address: NominatimAddress,
}

#[derive(Debug, Serialize, Deserialize)]
struct NominatimAddress {
    city: Option<String>,
    town: Option<String>,
    village: Option<String>,
    suburb: Option<String>,
    county: Option<String>,
    state: Option<String>,
    country: Option<String>,
}

/// Performs reverse geocoding using the Nominatim API (OpenStreetMap)
/// 
/// Takes latitude and longitude coordinates and returns a formatted location string
/// like "Kilimani, Nairobi, Kenya" or just the full display_name if parsing fails
pub async fn reverse_geocode(latitude: f64, longitude: f64) -> Result<String, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("air_quality_monitoring_system")
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
    
    let url = format!(
        "https://nominatim.openstreetmap.org/reverse?format=json&lat={}&lon={}&zoom=18&addressdetails=1",
        latitude, longitude
    );
    
    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Respect Nominatim's usage policy by checking status
    if response.status() != reqwest::StatusCode::OK {
        return Err(format!("API returned error status: {}", response.status()));
    }
    
    let data: NominatimResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    // Try to create a simplified location string from the address components
    let location = format_location(&data.address);
    
    // If we couldn't extract a good location from the address components,
    // fall back to the full display_name
    if location.is_empty() {
        Ok(data.display_name)
    } else {
        Ok(location)
    }
}

/// Formats a location string from address components
/// 
/// Tries to create a string like "Suburb, City, Country" or "Town, County, Country"
/// depending on which fields are available
fn format_location(address: &NominatimAddress) -> String {
    // First part: try suburb, then village, then town
    let first_part = address.suburb
        .as_ref()
        .or(address.village.as_ref())
        .or(address.town.as_ref());
    
    // Second part: try city, then county
    let second_part = address.city
        .as_ref()
        .or(address.county.as_ref());
    
    // Third part: try state, then country
    let third_part = address.state
        .as_ref()
        .or(address.country.as_ref());
    
    // Combine the parts that are available
    let mut parts = Vec::new();
    
    if let Some(part) = first_part {
        parts.push(part.clone());
    }
    
    if let Some(part) = second_part {
        // Only add if different from first part
        if first_part.map_or(true, |fp| fp != part) {
            parts.push(part.clone());
        }
    }
    
    if let Some(part) = third_part {
        // Only add if different from second part
        if second_part.map_or(true, |sp| sp != part) {
            parts.push(part.clone());
        }
    }
    
    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_reverse_geocode() {
        // This test is commented out to avoid making actual API calls during tests
        // Uncomment to test manually
        /*
        let result = reverse_geocode(37.7749, -122.4194).await;
        assert!(result.is_ok());
        let location = result.unwrap();
        println!("Location: {}", location);
        assert!(!location.is_empty());
        */
    }
}
