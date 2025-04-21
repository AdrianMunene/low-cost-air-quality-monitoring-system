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
                        if let Ok(timestamp) = parse_timestamp(&record.timestamp) {
                            if let Some(val) = record.temperature {
                                series_temp.push(DataPoint { timestamp, value: val });
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

                    // Determine the y-axis range
                    let y_min = series_temp.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
                    let y_max = series_temp.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
                    let y_range: Range<f64> = if y_min < y_max { y_min..y_max } else { 0.0..1.0 };

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
        <div>
            {
                if let Some(config) = &*chart_config {
                    html! { <TimeSeriesChart config={config.config.clone()} /> }
                } else {
                    html! { <p>{ "Loading temperature data..." }</p> }
                }
            }
        </div>
    }
}