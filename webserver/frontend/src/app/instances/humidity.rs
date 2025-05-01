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

#[function_component(HumidityChart)]
pub fn humidity_chart() -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);

    {
        let chart_config = chart_config.clone();
        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    let series_humidity = build_series(&fetched_data, 
                        |record| record.humidity, 
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    let x_range = compute_time_range(&series_humidity);
                    let y_range = compute_value_range(
                        series_humidity.iter()
                        .map(|point| point.value)
                    );

                    let chart_series_humidity = ChartSeries {
                        label: "Humidity".to_string(),
                        data: Rc::new(series_humidity),
                        color: BLUE,
                    };

                    let config = TimeSeriesChartConfig {
                        caption: "Humidity".to_string(),
                        x_desc: "Time".to_string(),
                        y_desc: "Humidity (%)".to_string(),
                        x_labels: 10,
                        x_range,
                        y_range,
                        series: vec![chart_series_humidity],
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
                html! { <div class="chart-loading">{ "Loading humidity data..." }</div> }
            }
        }
    </div>
    }
    
}