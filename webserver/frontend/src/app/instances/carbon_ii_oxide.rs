use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use chrono::{DateTime, Utc};
use crate::app::utils::air_quality_client::get_air_quality_data;
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

#[function_component(CarbonIIOxideChart)]
pub fn carbon_ii_oxide_chart() -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);

    {
        let chart_config = chart_config.clone();
        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    let mut series_co: Vec<DataPoint> = Vec::new();

                    for record in fetched_data.into_iter() {
                        match parse_timestamp(&record.timestamp) {
                            Ok(timestamp) => {
                                if let Some(val) = record.co {
                                    series_co.push(DataPoint { timestamp, value: val });
                                }
                            },
                            Err(e) => {
                                log::warn!("Failed to parse timestamp: {} - Error: {}", record.timestamp, e);
                            }
                        }
                    }

                    // Sort series by timestamp
                    series_co.sort_by_key(|p| p.timestamp);

                    // Determine the x-axis range with better distribution
                    let (x_min, x_max) = if !series_co.is_empty() {
                        let first_time = series_co.first().unwrap().timestamp;
                        let last_time = series_co.last().unwrap().timestamp;

                        // Calculate the time span
                        let time_span = last_time - first_time;

                        // Add 5% padding on both sides to prevent clustering at edges
                        let padding = time_span.num_seconds() as i64 * 5 / 100;
                        let padding_duration = chrono::Duration::seconds(padding);

                        (first_time - padding_duration, last_time + padding_duration)
                    } else {
                        (Utc::now(), Utc::now())
                    };

                    // Determine the y-axis range with padding
                    let y_min = if series_co.is_empty() {
                        0.0
                    } else {
                        let min = series_co.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
                        // Add 10% padding at the bottom, but don't go below zero for CO values
                        (min * 0.9).max(0.0)
                    };

                    let y_max = if series_co.is_empty() {
                        50.0 // Default max if no data (typical range for ambient monitoring)
                    } else {
                        let max = series_co.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
                        // Add 10% padding at the top
                        max * 1.1
                    };

                    let y_range: Range<f64> = y_min..y_max;

                    // Build chart series
                    let chart_series_co = ChartSeries {
                        label: "Carbon (II) Oxide".to_string(),
                        data: series_co,
                        color: RGBColor(220, 20, 60),
                    };

                    // Build the overall chart configuration
                    let config = TimeSeriesChartProps {
                        config: TimeSeriesChartConfig {
                            caption: "Carbon (II) Oxide Levels".to_string(),
                            x_desc: "Time".to_string(),
                            y_desc: "CO (ppm)".to_string(),
                            x_labels: 10,
                            x_range: x_min..x_max,
                            y_range,
                            series: vec![chart_series_co],
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
                    html! { <div class="chart-loading">{ "Loading CO data..." }</div> }
                }
            }
        </div>
    }
}