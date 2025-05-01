use chrono::{DateTime, Utc};
use crate::app::components::time_series_chart::DataPoint;

pub fn build_series<T, F1, F2>(records: &[T], value_function: F1, timestamp_function: F2) -> Vec<DataPoint> 
where
    F1: Fn(&T) -> Option<f64>,
    F2: Fn(&T) -> DateTime<Utc>,
{
    let mut series = Vec::new();

    for record in records {
        if let Some(value) = value_function(record) {
            let timestamp = timestamp_function(record); 
            series.push(DataPoint { timestamp, value })
        }
    }

    series.sort_by_key(|point| point.timestamp);

    series
}