use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::app::utils::air_quality_client::get_air_quality_data;
use crate::app::utils::time_filter::TimeRange;
use crate::app::utils::location_filter::{LocationFilter, filter_data_by_location};
use crate::app::utils::aqi_calculator::calculate_overall_aqi;
use crate::app::components::aqi_display::{AqiDisplay, AqiDisplayProps};

#[derive(Properties, Clone, PartialEq)]
pub struct AqiMetricsProps {
    pub time_range: TimeRange,
    #[prop_or_else(|| LocationFilter::MostRecent)]
    pub location_filter: LocationFilter,
}

#[function_component(AqiMetrics)]
pub fn aqi_metrics(props: &AqiMetricsProps) -> Html {
    let aqi_result = use_state(|| None);
    let is_loading = use_state(|| true);
    let time_range = props.time_range.clone();
    let location_filter = props.location_filter.clone();

    // Fetch data and calculate AQI
    {
        let aqi_result = aqi_result.clone();
        let is_loading = is_loading.clone();
        let location_filter = location_filter.clone();

        use_effect_with((time_range.clone(), location_filter.clone()), move |(time_range, location_filter)| {
            let time_range = time_range.clone();
            let location_filter = location_filter.clone();
            is_loading.set(true);
            aqi_result.set(None);

            spawn_local(async move {
                match get_air_quality_data().await {
                    Ok(data) => {
                        // Calculate AQI using the data, time range, and location filter
                        let result = calculate_overall_aqi(&data, &time_range, &location_filter);
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
