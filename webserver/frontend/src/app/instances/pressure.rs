use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::app::utils::parse_timestamp::parse_timestamp;
use crate::app::utils::air_quality_client::get_air_quality_data;
use crate::app::utils::{series_builder::build_series, range_calculations::{compute_time_range, compute_value_range}};
use crate::app::components::time_series_chart::{
    TimeSeriesChart,
    TimeSeriesChartProps,
    TimeSeriesChartConfig,
    ChartSeries,
};
use crate::app::utils::time_filter::{TimeRange, filter_data_by_time_range};
use std::rc::Rc;
use plotters::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PressureChartProps {
    pub time_range: TimeRange,
}

#[function_component(PressureChart)]
pub fn pressure_chart(props: &PressureChartProps) -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);
    let time_range = props.time_range.clone();

    {
        let chart_config = chart_config.clone();
        let time_range = time_range.clone();

        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    // Filter data by time range
                    let filtered_data = filter_data_by_time_range(
                        &fetched_data,
                        &time_range,
                        |record| parse_timestamp(&record.timestamp).ok()
                    );

                    log::info!("Filtered data for Pressure chart: {} records", filtered_data.len());

                    if filtered_data.is_empty() {
                        log::warn!("No data available for the selected time range");
                        chart_config.set(None);
                        return;
                    }

                    let series_pressure = build_series(&filtered_data,
                        |record| record.pressure,
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    // If series is empty after filtering, show no data
                    if series_pressure.is_empty() {
                        log::warn!("No pressure data available for the selected time range");
                        chart_config.set(None);
                        return;
                    }

                    let x_range = compute_time_range(&series_pressure);
                    let y_range = compute_value_range(
                        series_pressure.iter()
                        .map(|point| point.value)
                    );

                    let chart_series_pressure = ChartSeries {
                        label: "Pressure".to_string(),
                        data: Rc::new(series_pressure),
                        color: MAGENTA,
                    };

                    let config = TimeSeriesChartConfig {
                        caption: "Pressure".to_string(),
                        x_desc: "Time".to_string(),
                        y_desc: "Pressure (hPa)".to_string(),
                        x_labels: 10,
                        x_range,
                        y_range,
                        series: vec![chart_series_pressure],
                    };

                    let chart_props = TimeSeriesChartProps { config: Rc::new(config) };

                    chart_config.set(Some(chart_props));
                }

                Err(err) => {
                    log::error!("Failed to fetch air quality data: {}", err);
                }
            }
        });
    }

    // Re-fetch data when time range changes
    {
        let chart_config = chart_config.clone();
        use_effect_with(time_range, move |_| {
            chart_config.set(None); // Reset chart to show loading state
            || ()
        });
    }

    html! {
    <div class="chart-wrapper">
        {
            if let Some(config) = &*chart_config {
                html! { <TimeSeriesChart config={config.config.clone()} /> }
            } else {
                html! { <div class="chart-loading">{ "Loading Pressure data..." }</div> }
            }
        }
    </div>
    }
}