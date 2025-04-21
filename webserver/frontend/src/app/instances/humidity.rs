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
    DateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S").map(|dt| dt.with_timezone(&Utc))
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
                        if let Ok(timestamp) = parse_timestamp(&record.timestamp) {
                            if let Some(val) = record.humidity {
                                series_humidity.push(DataPoint { timestamp, value: val });
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

                    // Determine the y-axis range
                    let y_min = series_humidity.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
                    let y_max = series_humidity.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
                    let y_range: Range<f64> = if y_min < y_max { y_min..y_max } else { 0.0..100.0 }; // Default to 0-100% for humidity

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
        <div>
            {
                if let Some(config) = &*chart_config {
                    html! { <TimeSeriesChart config={config.config.clone()} /> }
                } else {
                    html! { <p>{ "Loading humidity data..." }</p> }
                }
            }
        </div>
    }
}