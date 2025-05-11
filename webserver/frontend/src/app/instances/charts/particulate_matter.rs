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
use crate::app::utils::location_filter::{LocationFilter, filter_data_by_location};
use std::rc::Rc;
use plotters::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ParticulateMatterChartProps {
    pub time_range: TimeRange,
    #[prop_or_else(|| LocationFilter::MostRecent)]
    pub location_filter: LocationFilter,
}

#[function_component(ParticulateMatterChart)]
pub fn particulate_matter_chart(props: &ParticulateMatterChartProps) -> Html {
    let chart_config = use_state(|| None::<TimeSeriesChartProps>);
    let time_range = props.time_range.clone();
    let location_filter = props.location_filter.clone();

    {
        let chart_config = chart_config.clone();
        let time_range = time_range.clone();
        let location_filter = location_filter.clone();

        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    // First filter by time range
                    let time_filtered_data = filter_data_by_time_range(
                        &fetched_data,
                        &time_range,
                        |record| parse_timestamp(&record.timestamp).ok()
                    );

                    // Then filter by location
                    let filtered_data = filter_data_by_location(
                        &time_filtered_data,
                        &location_filter,
                        |record| record.location.clone(),
                        |record| parse_timestamp(&record.timestamp).ok()
                    );

                    log::info!("Filtered data for PM chart: {} records", filtered_data.len());

                    if filtered_data.is_empty() {
                        log::warn!("No data available for the selected time range");
                        chart_config.set(None);
                        return;
                    }

                    let series_pm1 = build_series(&filtered_data,
                        |record| record.pm1_0,
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    let series_pm2_5 = build_series(&filtered_data,
                        |record| record.pm2_5,
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    let series_pm10 = build_series(&filtered_data,
                        |record| record.pm10,
                        |record| parse_timestamp(&record.timestamp).unwrap()
                    );

                    // If all series are empty after filtering, show no data
                    if series_pm1.is_empty() && series_pm2_5.is_empty() && series_pm10.is_empty() {
                        log::warn!("No PM data available for the selected time range");
                        chart_config.set(None);
                        return;
                    }

                    // Use the series with most data points for x_range calculation
                    let series_for_range = if !series_pm2_5.is_empty() {
                        &series_pm2_5
                    } else if !series_pm1.is_empty() {
                        &series_pm1
                    } else {
                        &series_pm10
                    };

                    let x_range = compute_time_range(series_for_range);
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

    // Re-fetch data when time range or location changes
    {
        let chart_config_time = chart_config.clone();
        use_effect_with(time_range, move |_| {
            chart_config_time.set(None); // Reset chart to show loading state
            || ()
        });

        let chart_config_location = chart_config.clone();
        use_effect_with(location_filter, move |_| {
            chart_config_location.set(None); // Reset chart to show loading state
            || ()
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