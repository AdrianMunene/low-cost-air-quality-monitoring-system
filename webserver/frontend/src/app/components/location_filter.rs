use yew::prelude::*;
use web_sys::HtmlSelectElement;
use crate::app::utils::location_filter::{LocationFilter, get_unique_locations, get_most_recent_location};
use wasm_bindgen_futures::spawn_local;
use crate::app::utils::air_quality_client::get_air_quality_data;
use crate::app::utils::parse_timestamp::parse_timestamp;

#[derive(Properties, Clone, PartialEq)]
pub struct LocationFilterProps {
    pub selected_location: LocationFilter,
    pub on_location_change: Callback<LocationFilter>,
}

#[function_component(LocationFilterComponent)]
pub fn location_filter(props: &LocationFilterProps) -> Html {
    // State for available locations
    let locations = use_state(|| Vec::<String>::new());
    let most_recent_location = use_state(|| None::<String>);
    let is_loading = use_state(|| true);

    // Fetch available locations on component mount
    {
        let locations = locations.clone();
        let most_recent_location = most_recent_location.clone();
        let is_loading = is_loading.clone();
        let on_location_change = props.on_location_change.clone();
        // Clone the selected_location to avoid borrowing props in the async block
        let selected_location = props.selected_location.clone();

        use_effect_with((), move |_| {
            is_loading.set(true);

            spawn_local(async move {
                match get_air_quality_data().await {
                    Ok(data) => {
                        // Get unique locations
                        let unique_locations = get_unique_locations(&data);
                        locations.set(unique_locations);

                        // Get most recent location
                        let recent_location = get_most_recent_location(
                            &data,
                            |record| record.location.clone(),
                            |record| parse_timestamp(&record.timestamp).ok()
                        );

                        // Set most recent location and update filter if needed
                        if let Some(location) = recent_location.clone() {
                            most_recent_location.set(Some(location));

                            // If current selection is MostRecent, update with the actual location
                            if matches!(selected_location, LocationFilter::MostRecent) {
                                on_location_change.emit(LocationFilter::MostRecent);
                            }
                        }

                        is_loading.set(false);
                    },
                    Err(e) => {
                        log::error!("Failed to fetch locations: {}", e);
                        is_loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    // Handle location selection change
    let on_select = {
        let on_location_change = props.on_location_change.clone();

        Callback::from(move |e: Event| {
            let select = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = select {
                let value = select.value();

                let location_filter = if value == "most_recent" {
                    LocationFilter::MostRecent
                } else {
                    LocationFilter::Specific(value)
                };

                on_location_change.emit(location_filter);
            }
        })
    };

    // Determine which option should be selected
    let selected_value = match &props.selected_location {
        LocationFilter::MostRecent => "most_recent",
        LocationFilter::Specific(location) => location,
    };

    // Get the display name for the most recent location
    let most_recent_display = most_recent_location.as_ref()
        .map(|loc| format!("Most Recent ({})", loc))
        .unwrap_or_else(|| "Most Recent Location".to_string());

    html! {
        <div class="location-filter">
            <div class="location-filter-row">
                <label for="location-filter" class="location-filter-label">{"Location:"}</label>
                <select
                    id="location-filter"
                    value={selected_value.to_string()}
                    onchange={on_select}
                    class="location-filter-select"
                    disabled={*is_loading}
                >
                    <option value="most_recent">{most_recent_display}</option>
                    {
                        locations.iter().map(|location| {
                            html! {
                                <option value={location.clone()}>{location.clone()}</option>
                            }
                        }).collect::<Html>()
                    }
                </select>
            </div>
        </div>
    }
}
