use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use chrono::{DateTime, Utc};
use crate::app::utils::air_quality_client::get_air_quality_data;
use crate::app::components::time_series_chart::{
    TimeSeriesChart, TimeSeriesChartProps, TimeSeriesChartConfig, ChartSeries, DataPoint,
};
use plotters::prelude::*; // For BLUE, GREEN, RED
use std::ops::Range;

fn parse_timestamp(ts: &str) -> Result<DateTime<Utc>, chrono::format::ParseError> {
    // Try parsing with different formats
    let result = DateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S %z");
    if result.is_ok() {
        return result.map(|dt| dt.with_timezone(&Utc));
    }

    // Try without timezone
    DateTime::parse_from_str(&format!("{} +0000", ts), "%Y-%m-%d %H:%M:%S %z")
        .map(|dt| dt.with_timezone(&Utc))
}

#[function_component(ParticulateMatterChart)]
pub fn particulate_matter_chart() -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);

    {
        let chart_config = chart_config.clone();
        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    let mut series_pm1: Vec<DataPoint> = Vec::new();
                    let mut series_pm2: Vec<DataPoint> = Vec::new();
                    let mut series_pm10: Vec<DataPoint> = Vec::new();

                    for record in fetched_data.into_iter() {
                        match parse_timestamp(&record.timestamp) {
                            Ok(timestamp) => {
                                if let Some(val) = record.pm1_0 {
                                    series_pm1.push(DataPoint { timestamp, value: val });
                                }
                                if let Some(val) = record.pm2_5 {
                                    series_pm2.push(DataPoint { timestamp, value: val });
                                }
                                if let Some(val) = record.pm10 {
                                    series_pm10.push(DataPoint { timestamp, value: val });
                                }
                            },
                            Err(e) => {
                                log::warn!("Failed to parse timestamp: {} - Error: {}", record.timestamp, e);
                            }
                        }
                    }

                    // Sort each series by timestamp.
                    series_pm1.sort_by_key(|p| p.timestamp);
                    series_pm2.sort_by_key(|p| p.timestamp);
                    series_pm10.sort_by_key(|p| p.timestamp);

                    // Determine the x-axis range with better distribution
                    let (x_min, x_max) = if !series_pm2.is_empty() {
                        let first_time = series_pm2.first().unwrap().timestamp;
                        let last_time = series_pm2.last().unwrap().timestamp;

                        // Calculate the time span
                        let time_span = last_time - first_time;

                        // Add 5% padding on both sides to prevent clustering at edges
                        let padding = time_span.num_seconds() as i64 * 5 / 100;
                        let padding_duration = chrono::Duration::seconds(padding);

                        (first_time - padding_duration, last_time + padding_duration)
                    } else {
                        (Utc::now(), Utc::now())
                    };

                    // Determine the y-axis range across all series.
                    let all_values: Vec<f64> = series_pm1.iter()
                        .chain(series_pm2.iter())
                        .chain(series_pm10.iter())
                        .map(|p| p.value)
                        .collect();

                    // Calculate y-axis range with padding
                    let y_min = if all_values.is_empty() {
                        0.0
                    } else {
                        let min = all_values.iter().cloned().fold(f64::INFINITY, f64::min);
                        // Add 10% padding at the bottom, but don't go below zero for PM values
                        (min * 0.9).max(0.0)
                    };

                    let y_max = if all_values.is_empty() {
                        10.0 // Default max if no data
                    } else {
                        let max = all_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                        // Add 10% padding at the top
                        max * 1.1
                    };

                    let y_range: Range<f64> = y_min..y_max;

                    // Build chart series.
                    let chart_series_pm1 = ChartSeries {
                        label: "PM 1.0µm".to_string(),
                        data: series_pm1,
                        color: RGBColor(50, 142, 110),
                    };
                    let chart_series_pm2 = ChartSeries {
                        label: "PM 2.5µm".to_string(),
                        data: series_pm2,
                        color: RGBColor(255, 165, 0),
                    };
                    let chart_series_pm10 = ChartSeries {
                        label: "PM 10µm".to_string(),
                        data: series_pm10,
                        color: RGBColor(59, 130, 246),
                    };

                    // Build the overall chart configuration.
                    let config = TimeSeriesChartProps {
                        config: TimeSeriesChartConfig {
                            caption: "Particulate Matter Metrics".to_string(),
                            x_desc: "Time".to_string(),
                            y_desc: "Concentration (µg/m³)".to_string(),
                            x_labels: 10,
                            x_range: x_min..x_max,
                            y_range,
                            series: vec![chart_series_pm1, chart_series_pm2, chart_series_pm10],
                        }
                    };

                    chart_config.set(Some(config));
                }
                Err(err) => {
                    log::error!("Failed to fetch air quality data: {}", err);
                }
            }
        });
    }

    html! {
        <div class="chart-wrapper">
            {
                if let Some(config) = &*chart_config {
                    html! { <TimeSeriesChart config={config.config.clone()} /> }
                } else {
                    html! { <div class="chart-loading">{ "Loading particulate matter data..." }</div> }
                }
            }
        </div>
    }
}
