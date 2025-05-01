use crate::app::components::time_series_chart::DataPoint;
use chrono::{DateTime, Utc};
use std::ops::Range;

pub fn compute_time_range(points: &[DataPoint]) -> Range<DateTime<Utc>>{
    if !points.is_empty() {
        let start_date = points.first().unwrap().timestamp;
        let end_date = points.last().unwrap().timestamp;
        let time_span = end_date - start_date;
        let padding = chrono::Duration::seconds(time_span.num_seconds() as i64 * 5 / 100);
        start_date - padding..end_date + padding
    } else {
        Utc::now()..Utc::now()
    }
}

pub fn compute_value_range<I: Iterator<Item=f64>>(iters: I) -> Range<f64> {
    let values: Vec<f64> = iters.collect(); 
    if !values.is_empty() {
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        min * 0.9..max * 1.1
    } else {
        0.0..100.0
    }
}