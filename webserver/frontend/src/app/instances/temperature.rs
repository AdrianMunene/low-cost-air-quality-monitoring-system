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

#[function_component(TemperatureChart)]
pub fn temperature_chart() -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);

    {
        let chart_config = chart_config.clone();
        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    let mut series_temp: Vec<DataPoint> = Vec::new();

                    for record in fetched_data.into_iter() {
                        match parse_timestamp(&record.timestamp) {
                            Ok(timestamp) => {
                                if let Some(val) = record.temperature {
                                    series_temp.push(DataPoint { timestamp, value: val });
                                }
                            },
                            Err(e) => {
                                log::warn!("Failed to parse timestamp: {} - Error: {}", record.timestamp, e);
                            }
                        }
                    }

                    if !series_temp.is_empty() {
                        // Sort series by timestamp
                        series_temp.sort_by_key(|p| p.timestamp);

                        // Determine the x-axis range with better distribution
                        let first_time = series_temp.first().unwrap().timestamp;
                        let last_time = series_temp.last().unwrap().timestamp;

                        // Calculate the time span
                        let time_span = last_time - first_time;

                        // Add 5% padding on both sides to prevent clustering at edges
                        let padding = time_span.num_seconds() as i64 * 5 / 100;
                        let padding_duration = chrono::Duration::seconds(padding);

                        let x_min = first_time - padding_duration;
                        let x_max = last_time + padding_duration;

                        // Determine the y-axis range with padding
                        let min = series_temp.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
                        let max = series_temp.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
                        
                        // Add padding to y-axis range
                        let y_min = (min * 0.9).max(0.0); // 10% padding, don't go below zero
                        let y_max = max * 1.1; // 10% padding at the top

                        let y_range: Range<f64> = y_min..y_max;

                        // Build chart series
                        let chart_series_temp = ChartSeries {
                            label: "Temperature".to_string(),
                            data: series_temp,
                            color: RED,
                        };

                        // Build the overall chart configuration
                        let config = TimeSeriesChartProps {
                            config: TimeSeriesChartConfig {
                                caption: "Temperature Measurements".to_string(),
                                x_desc: "Time".to_string(),
                                y_desc: "Temperature (°C)".to_string(),
                                x_labels: 10,
                                x_range: x_min..x_max,
                                y_range,
                                series: vec![chart_series_temp],
                            }
                        };

                        chart_config.set(Some(config));
                    }
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
                    html! { <div class="chart-loading">{ "Loading temperature data..." }</div> }
                }
            }
        </div>
    }
}
