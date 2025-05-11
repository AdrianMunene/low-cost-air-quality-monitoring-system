use yew::prelude::*;
use web_sys::HtmlSelectElement;
use crate::app::utils::location_filter::LocationFilter;
use wasm_bindgen_futures::spawn_local;
use crate::app::utils::air_quality_client::get_air_quality_data;
use crate::app::utils::location_filter::get_unique_locations;

#[derive(Properties, Clone, PartialEq)]
pub struct LocationFilterProps {
    pub selected_location: LocationFilter,
    pub on_location_change: Callback<LocationFilter>,
}

#[function_component(LocationFilterComponent)]
pub fn location_filter(props: &LocationFilterProps) -> Html {
    // State for available locations
    let locations = use_state(|| Vec::<String>::new());
    let is_loading = use_state(|| true);

    // Fetch available locations on component mount
    {
        let locations = locations.clone();
        let is_loading = is_loading.clone();

        use_effect_with((), move |_| {
            is_loading.set(true);
            
            spawn_local(async move {
                match get_air_quality_data().await {
                    Ok(data) => {
                        let unique_locations = get_unique_locations(&data);
                        locations.set(unique_locations);
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
                
                let location_filter = if value == "all" {
                    LocationFilter::All
                } else {
                    LocationFilter::Specific(value)
                };
                
                on_location_change.emit(location_filter);
            }
        })
    };

    // Determine which option should be selected
    let selected_value = match &props.selected_location {
        LocationFilter::All => "all",
        LocationFilter::Specific(location) => location,
    };

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
                    <option value="all">{"All Locations"}</option>
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
