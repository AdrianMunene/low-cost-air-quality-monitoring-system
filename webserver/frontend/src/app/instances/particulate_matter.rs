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

#[function_component(ParticulateMatterChart)]
pub fn particulate_matter_chart() -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);

    {
        let chart_config = chart_config.clone();
        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    let series_pm1 = build_series(&fetched_data, 
                        |record| record.pm1_0, 
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    let series_pm2_5 = build_series(&fetched_data, 
                        |record| record.pm2_5, 
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    let series_pm10 = build_series(&fetched_data, 
                        |record| record.pm10, 
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    let x_range = compute_time_range(&series_pm2_5);
                    let y_range = compute_value_range(
                        series_pm1.iter()
                        .chain(series_pm2_5.iter())
                        .chain(series_pm10.iter())
                        .map(|point| point.value)
                    );

                    let chart_series_pm1 = ChartSeries {
                        label: "PM 1.0µm".to_string(),
                        data: Rc::new(series_pm1),
                        color: RGBColor(0, 128, 128),
                    };

                    let chart_series_pm2_5 = ChartSeries {
                        label: "PM 2.5µm".to_string(),
                        data: Rc::new(series_pm2_5),
                        color: RGBColor(255, 165, 0),
                    };

                    let chart_series_pm10 = ChartSeries {
                        label: "PM 10µm".to_string(),
                        data: Rc::new(series_pm10),
                        color: RGBColor(59, 130, 246),
                    };

                    let config = TimeSeriesChartConfig {
                        caption: "Particulate Matter".to_string(),
                        x_desc: "Time".to_string(),
                        y_desc: "Concentration (µg/m³)".to_string(),
                        x_labels: 10,
                        x_range,
                        y_range,
                        series: vec![chart_series_pm1, chart_series_pm2_5, chart_series_pm10],
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
                html! { <div class="chart-loading">{ "Loading Particulate Matter data..." }</div> }
            }
        }
    </div>
    }
}