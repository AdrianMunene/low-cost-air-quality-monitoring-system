use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::app::utils::air_quality_client::get_air_quality_data;
use crate::app::utils::time_filter::TimeRange;
use crate::app::utils::aqi_calculator::calculate_overall_aqi;
use crate::app::components::aqi_display::{AqiDisplay, AqiDisplayProps};

#[derive(Properties, Clone, PartialEq)]
pub struct AqiMetricsProps {
    pub time_range: TimeRange,
}

#[function_component(AqiMetrics)]
pub fn aqi_metrics(props: &AqiMetricsProps) -> Html {
    let aqi_result = use_state(|| None);
    let is_loading = use_state(|| true);
    let time_range = props.time_range.clone();

    // Fetch data and calculate AQI
    {
        let aqi_result = aqi_result.clone();
        let is_loading = is_loading.clone();

        use_effect_with(time_range.clone(), move |time_range| {
            let time_range = time_range.clone();
            is_loading.set(true);
            aqi_result.set(None);

            spawn_local(async move {
                match get_air_quality_data().await {
                    Ok(data) => {
                        let result = calculate_overall_aqi(&data, &time_range);
                        aqi_result.set(result);
                        is_loading.set(false);
                    },
                    Err(err) => {
                        log::error!("Failed to fetch air quality data for AQI: {}", err);
                        is_loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    html! {
        <AqiDisplay
            aqi_result={(*aqi_result).clone()}
            is_loading={*is_loading}
        />
    }
}
