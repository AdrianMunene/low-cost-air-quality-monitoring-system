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
pub struct CarbonIVOxideChartProps {
    pub time_range: TimeRange,
}

#[function_component(CarbonIVOxideChart)]
pub fn carbon_iv_oxide_chart(props: &CarbonIVOxideChartProps) -> Html {
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

                    log::info!("Filtered data for CO2 chart: {} records", filtered_data.len());

                    if filtered_data.is_empty() {
                        log::warn!("No data available for the selected time range");
                        chart_config.set(None);
                        return;
                    }

                    let series_co2 = build_series(&filtered_data,
                        |record| record.co2,
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    // If series is empty after filtering, show no data
                    if series_co2.is_empty() {
                        log::warn!("No CO2 data available for the selected time range");
                        chart_config.set(None);
                        return;
                    }

                    let x_range = compute_time_range(&series_co2);
                    let y_range = compute_value_range(
                        series_co2.iter()
                        .map(|point| point.value)
                    );

                    let chart_series_co2 = ChartSeries {
                        label: "Carbon (IV) Oxide".to_string(),
                        data: Rc::new(series_co2),
                        color: RGBColor(34, 139, 34),
                    };

                    let config = TimeSeriesChartConfig {
                        caption: "Carbon (IV) Oxide".to_string(),
                        x_desc: "Time".to_string(),
                        y_desc: "CO₂ (ppm)".to_string(),
                        x_labels: 10,
                        x_range,
                        y_range,
                        series: vec![chart_series_co2],
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
                html! { <div class="chart-loading">{ "Loading Carbon (IV) Oxide data..." }</div> }
            }
        }
    </div>
    }
}