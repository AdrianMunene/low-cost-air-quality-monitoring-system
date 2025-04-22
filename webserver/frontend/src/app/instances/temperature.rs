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

                    // Sort series by timestamp
                    series_temp.sort_by_key(|p| p.timestamp);

                    // Determine the x-axis range
                    let (x_min, x_max) = if !series_temp.is_empty() {
                        (series_temp.first().unwrap().timestamp, series_temp.last().unwrap().timestamp)
                    } else {
                        (Utc::now(), Utc::now())
                    };

                    // Determine the y-axis range with padding
                    let y_min = if series_temp.is_empty() {
                        0.0
                    } else {
                        let min = series_temp.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
                        // Add 10% padding at the bottom, but don't go below zero for temperature values
                        (min * 0.9).max(0.0)
                    };

                    let y_max = if series_temp.is_empty() {
                        30.0 // Default max if no data
                    } else {
                        let max = series_temp.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
                        // Add 10% padding at the top
                        max * 1.1
                    };

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