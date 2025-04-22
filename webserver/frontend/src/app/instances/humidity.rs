use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use chrono::{DateTime, Utc};
use crate::app::air_quality_client::get_air_quality_data;
use crate::app::components::time_series_chart::{
    TimeSeriesChart, TimeSeriesChartProps, TimeSeriesChartConfig, ChartSeries, DataPoint,
};
use plotters::prelude::*;
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

#[function_component(HumidityChart)]
pub fn humidity_chart() -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);

    {
        let chart_config = chart_config.clone();
        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    let mut series_humidity: Vec<DataPoint> = Vec::new();

                    for record in fetched_data.into_iter() {
                        match parse_timestamp(&record.timestamp) {
                            Ok(timestamp) => {
                                if let Some(val) = record.humidity {
                                    series_humidity.push(DataPoint { timestamp, value: val });
                                }
                            },
                            Err(e) => {
                                log::warn!("Failed to parse timestamp: {} - Error: {}", record.timestamp, e);
                            }
                        }
                    }

                    // Sort series by timestamp
                    series_humidity.sort_by_key(|p| p.timestamp);

                    // Determine the x-axis range
                    let (x_min, x_max) = if !series_humidity.is_empty() {
                        (series_humidity.first().unwrap().timestamp, series_humidity.last().unwrap().timestamp)
                    } else {
                        (Utc::now(), Utc::now())
                    };

                    // Determine the y-axis range with padding
                    let y_min = if series_humidity.is_empty() {
                        0.0
                    } else {
                        let min = series_humidity.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
                        // Add 10% padding at the bottom, but don't go below zero for humidity values
                        (min * 0.9).max(0.0)
                    };

                    let y_max = if series_humidity.is_empty() {
                        100.0 // Default max if no data (humidity is 0-100%)
                    } else {
                        let max = series_humidity.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
                        // Add 10% padding at the top, but cap at 100% for humidity
                        (max * 1.1).min(100.0)
                    };

                    let y_range: Range<f64> = y_min..y_max;

                    // Build chart series
                    let chart_series_humidity = ChartSeries {
                        label: "Relative Humidity".to_string(),
                        data: series_humidity,
                        color: CYAN,  // Using CYAN color for humidity
                    };

                    // Build the overall chart configuration
                    let config = TimeSeriesChartProps {
                        config: TimeSeriesChartConfig {
                            caption: "Relative Humidity Levels".to_string(),
                            x_desc: "Time".to_string(),
                            y_desc: "Humidity (%)".to_string(),
                            x_labels: 10,
                            x_range: x_min..x_max,
                            y_range,
                            series: vec![chart_series_humidity],
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
                    html! { <div class="chart-loading">{ "Loading humidity data..." }</div> }
                }
            }
        </div>
    }
}