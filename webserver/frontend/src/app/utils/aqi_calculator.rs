use crate::app::utils::air_quality_client::AirQualityData;
use crate::app::utils::time_filter::{TimeRange, filter_data_by_time_range};
use crate::app::utils::location_filter::{LocationFilter, filter_data_by_location};
use crate::app::utils::parse_timestamp::parse_timestamp;
use std::cmp::Ordering;

/// AQI category with color and description
#[derive(Clone, PartialEq, Debug)]
pub struct AqiCategory {
    pub name: String,
    pub color: String,
    pub description: String,
}

/// AQI result with value, category, and dominant pollutant
#[derive(Clone, PartialEq, Debug)]
pub struct AqiResult {
    pub value: i32,
    pub category: AqiCategory,
    pub dominant_pollutant: String,
}

/// Breakpoint structure for AQI calculation
struct Breakpoint {
    aqi_low: i32,
    aqi_high: i32,
    conc_low: f64,
    conc_high: f64,
}

/// Get AQI category based on AQI value
fn get_aqi_category(aqi: i32) -> AqiCategory {
    match aqi {
        0..=50 => AqiCategory {
            name: "Good".to_string(),
            color: "#00E400".to_string(), // Green
            description: "Air quality is satisfactory, and air pollution poses little or no risk.".to_string(),
        },
        51..=100 => AqiCategory {
            name: "Moderate".to_string(),
            color: "#FFFF00".to_string(), // Yellow
            description: "Air quality is acceptable. However, there may be a risk for some people, particularly those who are unusually sensitive to air pollution.".to_string(),
        },
        101..=150 => AqiCategory {
            name: "Unhealthy for Sensitive Groups".to_string(),
            color: "#FF7E00".to_string(), // Orange
            description: "Members of sensitive groups may experience health effects. The general public is less likely to be affected.".to_string(),
        },
        151..=200 => AqiCategory {
            name: "Unhealthy".to_string(),
            color: "#FF0000".to_string(), // Red
            description: "Some members of the general public may experience health effects; members of sensitive groups may experience more serious health effects.".to_string(),
        },
        201..=300 => AqiCategory {
            name: "Very Unhealthy".to_string(),
            color: "#99004C".to_string(), // Purple
            description: "Health alert: The risk of health effects is increased for everyone.".to_string(),
        },
        301..=500 => AqiCategory {
            name: "Hazardous".to_string(),
            color: "#7E0023".to_string(), // Maroon
            description: "Health warning of emergency conditions: everyone is more likely to be affected.".to_string(),
        },
        _ => AqiCategory {
            name: "Out of Range".to_string(),
            color: "#000000".to_string(), // Black
            description: "AQI value is out of range.".to_string(),
        },
    }
}

/// Calculate AQI for PM2.5
fn calculate_pm25_aqi(concentration: f64) -> i32 {
    let breakpoints = [
        Breakpoint { aqi_low: 0, aqi_high: 50, conc_low: 0.0, conc_high: 12.0 },
        Breakpoint { aqi_low: 51, aqi_high: 100, conc_low: 12.1, conc_high: 35.4 },
        Breakpoint { aqi_low: 101, aqi_high: 150, conc_low: 35.5, conc_high: 55.4 },
        Breakpoint { aqi_low: 151, aqi_high: 200, conc_low: 55.5, conc_high: 150.4 },
        Breakpoint { aqi_low: 201, aqi_high: 300, conc_low: 150.5, conc_high: 250.4 },
        Breakpoint { aqi_low: 301, aqi_high: 500, conc_low: 250.5, conc_high: 500.4 },
    ];

    calculate_aqi_for_pollutant(concentration, &breakpoints)
}

/// Calculate AQI for PM10
fn calculate_pm10_aqi(concentration: f64) -> i32 {
    let breakpoints = [
        Breakpoint { aqi_low: 0, aqi_high: 50, conc_low: 0.0, conc_high: 54.0 },
        Breakpoint { aqi_low: 51, aqi_high: 100, conc_low: 55.0, conc_high: 154.0 },
        Breakpoint { aqi_low: 101, aqi_high: 150, conc_low: 155.0, conc_high: 254.0 },
        Breakpoint { aqi_low: 151, aqi_high: 200, conc_low: 255.0, conc_high: 354.0 },
        Breakpoint { aqi_low: 201, aqi_high: 300, conc_low: 355.0, conc_high: 424.0 },
        Breakpoint { aqi_low: 301, aqi_high: 500, conc_low: 425.0, conc_high: 604.0 },
    ];

    calculate_aqi_for_pollutant(concentration, &breakpoints)
}

/// Calculate AQI for CO
fn calculate_co_aqi(concentration: f64) -> i32 {
    let breakpoints = [
        Breakpoint { aqi_low: 0, aqi_high: 50, conc_low: 0.0, conc_high: 4.4 },
        Breakpoint { aqi_low: 51, aqi_high: 100, conc_low: 4.5, conc_high: 9.4 },
        Breakpoint { aqi_low: 101, aqi_high: 150, conc_low: 9.5, conc_high: 12.4 },
        Breakpoint { aqi_low: 151, aqi_high: 200, conc_low: 12.5, conc_high: 15.4 },
        Breakpoint { aqi_low: 201, aqi_high: 300, conc_low: 15.5, conc_high: 30.4 },
        Breakpoint { aqi_low: 301, aqi_high: 500, conc_low: 30.5, conc_high: 50.4 },
    ];

    calculate_aqi_for_pollutant(concentration, &breakpoints)
}

/// Calculate AQI for O3 (8-hour average)
fn calculate_o3_aqi(concentration: f64) -> i32 {
    // Convert from ppb to ppm if needed (our data might be in ppb)
    let concentration_ppm = if concentration > 1.0 { concentration / 1000.0 } else { concentration };

    let breakpoints = [
        Breakpoint { aqi_low: 0, aqi_high: 50, conc_low: 0.000, conc_high: 0.054 },
        Breakpoint { aqi_low: 51, aqi_high: 100, conc_low: 0.055, conc_high: 0.070 },
        Breakpoint { aqi_low: 101, aqi_high: 150, conc_low: 0.071, conc_high: 0.085 },
        Breakpoint { aqi_low: 151, aqi_high: 200, conc_low: 0.086, conc_high: 0.105 },
        Breakpoint { aqi_low: 201, aqi_high: 300, conc_low: 0.106, conc_high: 0.200 },
    ];

    calculate_aqi_for_pollutant(concentration_ppm, &breakpoints)
}

/// Generic AQI calculation using the EPA formula
fn calculate_aqi_for_pollutant(concentration: f64, breakpoints: &[Breakpoint]) -> i32 {
    // If concentration is negative or NaN, return 0
    if concentration.is_nan() || concentration < 0.0 {
        return 0;
    }

    for bp in breakpoints {
        if concentration >= bp.conc_low && concentration <= bp.conc_high {
            // Apply the AQI formula
            let aqi = ((bp.aqi_high - bp.aqi_low) as f64 / (bp.conc_high - bp.conc_low))
                    * (concentration - bp.conc_low)
                    + (bp.aqi_low as f64);

            return aqi.round() as i32;
        }
    }

    // If concentration is above the highest breakpoint, return the highest AQI
    if concentration > breakpoints.last().unwrap().conc_high {
        return breakpoints.last().unwrap().aqi_high;
    }

    // Default to 0 if no match (should not happen with proper breakpoints)
    0
}

/// Calculate average concentration for a specific pollutant
fn calculate_average_concentration<F>(
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

/// Calculate AQI for all pollutants and return the overall AQI
pub fn calculate_overall_aqi(data: &[AirQualityData], time_range: &TimeRange, location_filter: &LocationFilter) -> Option<AqiResult> {
    // Calculate average concentrations
    let avg_pm25 = calculate_average_concentration(data, time_range, location_filter, |record| record.pm2_5);
    let avg_pm10 = calculate_average_concentration(data, time_range, location_filter, |record| record.pm10);
    let avg_co = calculate_average_concentration(data, time_range, location_filter, |record| record.co);
    let avg_o3 = calculate_average_concentration(data, time_range, location_filter, |record| record.o3);

    // Calculate AQI for each pollutant
    let mut aqi_values = Vec::new();

    if let Some(pm25) = avg_pm25 {
        let aqi = calculate_pm25_aqi(pm25);
        aqi_values.push((aqi, "PM2.5".to_string()));
    }

    if let Some(pm10) = avg_pm10 {
        let aqi = calculate_pm10_aqi(pm10);
        aqi_values.push((aqi, "PM10".to_string()));
    }

    if let Some(co) = avg_co {
        let aqi = calculate_co_aqi(co);
        aqi_values.push((aqi, "CO".to_string()));
    }

    if let Some(o3) = avg_o3 {
        let aqi = calculate_o3_aqi(o3);
        aqi_values.push((aqi, "O₃".to_string()));
    }

    // Find the maximum AQI value and its corresponding pollutant
    if aqi_values.is_empty() {
        return None;
    }

    aqi_values.sort_by(|a, b| {
        if a.0 == b.0 {
            Ordering::Equal
        } else if a.0 > b.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let (max_aqi, dominant_pollutant) = aqi_values.first().unwrap().clone();
    let category = get_aqi_category(max_aqi);

    Some(AqiResult {
        value: max_aqi,
        category,
        dominant_pollutant,
    })
}
