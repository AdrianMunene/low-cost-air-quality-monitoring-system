use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::app::utils::air_quality_client::{get_air_quality_data, AirQualityData};
use crate::app::utils::time_filter::TimeRange;
use crate::app::utils::location_filter::{LocationFilter, filter_data_by_location};
use crate::app::utils::average_calculator::calculate_average;
use crate::app::components::average_metrics::{AverageMetrics, AverageMetricsProps, MetricData};

#[derive(Properties, Clone, PartialEq)]
pub struct ParticulateMetricsProps {
    pub time_range: TimeRange,
    #[prop_or_else(|| LocationFilter::MostRecent)]
    pub location_filter: LocationFilter,
}

#[function_component(AverageParticulateMetrics)]
pub fn average_particulate_metrics(props: &ParticulateMetricsProps) -> Html {
    let metrics = use_state(|| Vec::<MetricData>::new());
    let is_loading = use_state(|| true);
    let time_range = props.time_range.clone();
    let location_filter = props.location_filter.clone();

    // Fetch data and calculate averages
    {
        let metrics = metrics.clone();
        let is_loading = is_loading.clone();
        let location_filter = location_filter.clone();

        use_effect_with((time_range.clone(), location_filter.clone()), move |(time_range, location_filter)| {
            let time_range = time_range.clone();
            let location_filter = location_filter.clone();
            is_loading.set(true);
            metrics.set(Vec::new());

            spawn_local(async move {
                match get_air_quality_data().await {
                    Ok(fetched_data) => {
                        let mut metrics_vec = Vec::new();

                        // Calculate PM1.0 average
                        if let Some(avg_pm1) = calculate_average(&fetched_data, &time_range, &location_filter, |record| record.pm1_0) {
                            metrics_vec.push(MetricData {
                                label: "PM1.0".to_string(),
                                value: avg_pm1,
                                unit: "µg/m³".to_string(),
                            });
                        }

                        // Calculate PM2.5 average
                        if let Some(avg_pm25) = calculate_average(&fetched_data, &time_range, &location_filter, |record| record.pm2_5) {
                            metrics_vec.push(MetricData {
                                label: "PM2.5".to_string(),
                                value: avg_pm25,
                                unit: "µg/m³".to_string(),
                            });
                        }

                        // Calculate PM10 average
                        if let Some(avg_pm10) = calculate_average(&fetched_data, &time_range, &location_filter, |record| record.pm10) {
                            metrics_vec.push(MetricData {
                                label: "PM10".to_string(),
                                value: avg_pm10,
                                unit: "µg/m³".to_string(),
                            });
                        }

                        metrics.set(metrics_vec);
                        is_loading.set(false);
                    },
                    Err(err) => {
                        log::error!("Failed to fetch air quality data: {}", err);
                        is_loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    html! {
        <AverageMetrics
            title="Particulate Matter"
            metrics={(*metrics).clone()}
            is_loading={*is_loading}
        />
    }
}
