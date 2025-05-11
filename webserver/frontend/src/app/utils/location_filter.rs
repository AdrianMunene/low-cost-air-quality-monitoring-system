use crate::app::utils::air_quality_client::AirQualityData;
use crate::app::utils::parse_timestamp::parse_timestamp;
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Debug)]
pub enum LocationFilter {
    MostRecent,         // Show data from the most recent location
    Specific(String),   // Show data from a specific location
}

impl LocationFilter {
    // Format for display
    pub fn display_name(&self) -> String {
        match self {
            LocationFilter::MostRecent => "Most Recent Location".to_string(),
            LocationFilter::Specific(location) => location.clone(),
        }
    }
}

// Helper function to filter data by location
pub fn filter_data_by_location<T, F, G>(
    data: &[T],
    location_filter: &LocationFilter,
    location_extractor: F,
    timestamp_extractor: G
) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> Option<String> + Copy,
    G: Fn(&T) -> Option<DateTime<Utc>> + Copy
{
    match location_filter {
        LocationFilter::MostRecent => {
            // Find the most recent location
            if let Some(most_recent_location) = get_most_recent_location(data, location_extractor, timestamp_extractor) {
                // Filter data to only include items from this location
                data.iter()
                    .filter_map(|item| {
                        if let Some(location) = location_extractor(item) {
                            if location == most_recent_location {
                                Some(item.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                // If no location found, return empty vector
                Vec::new()
            }
        },
        LocationFilter::Specific(target_location) => {
            data.iter()
                .filter_map(|item| {
                    if let Some(location) = location_extractor(item) {
                        if location == *target_location {
                            Some(item.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        }
    }
}

// Function to get the most recent location from data
pub fn get_most_recent_location<T, F, G>(
    data: &[T],
    location_extractor: F,
    timestamp_extractor: G
) -> Option<String>
where
    F: Fn(&T) -> Option<String> + Copy,
    G: Fn(&T) -> Option<DateTime<Utc>> + Copy
{
    // Find the most recent data point
    let mut most_recent_item: Option<&T> = None;
    let mut most_recent_time: Option<DateTime<Utc>> = None;

    for item in data {
        if let Some(timestamp) = timestamp_extractor(item) {
            if most_recent_time.is_none() || timestamp > most_recent_time.unwrap() {
                most_recent_time = Some(timestamp);
                most_recent_item = Some(item);
            }
        }
    }

    // Extract location from the most recent item
    most_recent_item.and_then(|item| location_extractor(item))
}

// Get unique locations from air quality data
pub fn get_unique_locations(data: &[AirQualityData]) -> Vec<String> {
    let mut locations = Vec::new();

    for item in data {
        if let Some(location) = &item.location {
            if !locations.contains(location) {
                locations.push(location.clone());
            }
        }
    }

    // Sort locations alphabetically
    locations.sort();

    locations
}
