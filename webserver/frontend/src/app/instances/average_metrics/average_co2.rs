use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::app::utils::air_quality_client::{get_air_quality_data, AirQualityData};
use crate::app::utils::time_filter::TimeRange;
use crate::app::utils::location_filter::{LocationFilter, filter_data_by_location};
use crate::app::utils::average_calculator::calculate_average;
use crate::app::components::average_metrics::{AverageMetrics, AverageMetricsProps, MetricData};

#[derive(Properties, Clone, PartialEq)]
pub struct CO2MetricsProps {
    pub time_range: TimeRange,
    #[prop_or_else(|| LocationFilter::All)]
    pub location_filter: LocationFilter,
}

#[function_component(AverageCO2Metrics)]
pub fn average_co2_metrics(props: &CO2MetricsProps) -> Html {
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

                        // Calculate CO2 average
                        if let Some(avg_co2) = calculate_average(&fetched_data, &time_range, &location_filter, |record| record.co2) {
                            metrics_vec.push(MetricData {
                                label: "Carbon Dioxide".to_string(),
                                value: avg_co2,
                                unit: "ppm".to_string(),
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
            title="Carbon Dioxide (CO₂)"
            metrics={(*metrics).clone()}
            is_loading={*is_loading}
        />
    }
}
