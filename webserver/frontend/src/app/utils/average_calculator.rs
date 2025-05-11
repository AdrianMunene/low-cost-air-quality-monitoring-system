use crate::app::utils::air_quality_client::AirQualityData;
use crate::app::utils::time_filter::{TimeRange, filter_data_by_time_range};
use crate::app::utils::location_filter::{LocationFilter, filter_data_by_location};
use crate::app::utils::parse_timestamp::parse_timestamp;

/// Calculate the average value for a specific metric from air quality data
pub fn calculate_average<F>(
    data: &[AirQualityData],
    time_range: &TimeRange,
    location_filter: &LocationFilter,
    value_extractor: F,
) -> Option<f64>
where
    F: Fn(&AirQualityData) -> Option<f64>,
{
    // First filter data by time range
    let time_filtered_data = filter_data_by_time_range(
        data,
        time_range,
        |record| parse_timestamp(&record.timestamp).ok(),
    );

    // Then filter by location
    let filtered_data = filter_data_by_location(
        &time_filtered_data,
        location_filter,
        |record| record.location.clone(),
        |record| parse_timestamp(&record.timestamp).ok(),
    );

    // Extract values and calculate average
    let values: Vec<f64> = filtered_data
        .iter()
        .filter_map(|record| value_extractor(record))
        .collect();

    if values.is_empty() {
        None
    } else {
        let sum: f64 = values.iter().sum();
        Some(sum / values.len() as f64)
    }
}

/// Calculate multiple averages at once
pub fn calculate_multiple_averages(
    data: &[AirQualityData],
    time_range: &TimeRange,
    location_filter: &LocationFilter,
) -> (
    Option<f64>, // temperature
    Option<f64>, // humidity
    Option<f64>, // pressure
    Option<f64>, // pm1_0
    Option<f64>, // pm2_5
    Option<f64>, // pm10
    Option<f64>, // co2
    Option<f64>, // co
    Option<f64>, // o3
) {
    (
        calculate_average(data, time_range, location_filter, |record| record.temperature),
        calculate_average(data, time_range, location_filter, |record| record.humidity),
        calculate_average(data, time_range, location_filter, |record| record.pressure),
        calculate_average(data, time_range, location_filter, |record| record.pm1_0),
        calculate_average(data, time_range, location_filter, |record| record.pm2_5),
        calculate_average(data, time_range, location_filter, |record| record.pm10),
        calculate_average(data, time_range, location_filter, |record| record.co2),
        calculate_average(data, time_range, location_filter, |record| record.co),
        calculate_average(data, time_range, location_filter, |record| record.o3),
    )
}
