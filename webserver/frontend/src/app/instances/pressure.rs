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
use std::rc::Rc;
use plotters::prelude::*;

#[function_component(PressureChart)]
pub fn pressure_chart() -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);

    {
        let chart_config = chart_config.clone();
        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    let series_pressure = build_series(&fetched_data, 
                        |record| record.pressure, 
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

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