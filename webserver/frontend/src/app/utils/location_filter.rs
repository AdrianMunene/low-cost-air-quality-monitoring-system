use crate::app::utils::air_quality_client::AirQualityData;

#[derive(Clone, PartialEq, Debug)]
pub enum LocationFilter {
    All,                // Show data from all locations
    Specific(String),   // Show data from a specific location
}

impl LocationFilter {
    // Format for display
    pub fn display_name(&self) -> String {
        match self {
            LocationFilter::All => "All Locations".to_string(),
            LocationFilter::Specific(location) => location.clone(),
        }
    }
}

// Helper function to filter data by location
pub fn filter_data_by_location<T, F>(
    data: &[T],
    location_filter: &LocationFilter,
    location_extractor: F
) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> Option<String>
{
    match location_filter {
        LocationFilter::All => data.to_vec(),
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
