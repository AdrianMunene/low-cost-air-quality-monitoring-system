use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::app::utils::air_quality_client::{get_air_quality_data, AirQualityData};
use crate::app::utils::time_filter::TimeRange;
use crate::app::utils::average_calculator::calculate_average;
use crate::app::components::average_metrics::{AverageMetrics, AverageMetricsProps, MetricData};

#[derive(Properties, Clone, PartialEq)]
pub struct O3MetricsProps {
    pub time_range: TimeRange,
}

#[function_component(AverageO3Metrics)]
pub fn average_o3_metrics(props: &O3MetricsProps) -> Html {
    let metrics = use_state(|| Vec::<MetricData>::new());
    let is_loading = use_state(|| true);
    let time_range = props.time_range.clone();

    // Fetch data and calculate averages
    {
        let metrics = metrics.clone();
        let is_loading = is_loading.clone();

        use_effect_with(time_range.clone(), move |time_range| {
            let time_range = time_range.clone();
            is_loading.set(true);
            metrics.set(Vec::new());

            spawn_local(async move {
                match get_air_quality_data().await {
                    Ok(data) => {
                        let mut metrics_vec = Vec::new();

                        // Calculate O3 average
                        if let Some(avg_o3) = calculate_average(&data, &time_range, |record| record.o3) {
                            metrics_vec.push(MetricData {
                                label: "Ozone".to_string(),
                                value: avg_o3,
                                unit: "ppb".to_string(),
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
            title="Ozone (O₃)"
            metrics={(*metrics).clone()}
            is_loading={*is_loading}
        />
    }
}
