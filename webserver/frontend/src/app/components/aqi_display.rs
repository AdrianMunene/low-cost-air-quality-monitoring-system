use yew::prelude::*;
use crate::app::utils::aqi_calculator::AqiResult;

// Calculate the position of the indicator based on AQI value
fn calculate_indicator_position(aqi: i32) -> f32 {
    // Position calculation based on AQI ranges
    // Good: 0-50, Moderate: 51-100, Unhealthy: 101+
    if aqi <= 50 {
        // Map 0-50 to 0-33%
        (aqi as f32 / 50.0) * 33.0
    } else if aqi <= 100 {
        // Map 51-100 to 33-66%
        33.0 + ((aqi as f32 - 50.0) / 50.0) * 33.0
    } else {
        // Map 101-500 to 66-100%
        66.0 + ((aqi as f32 - 100.0) / 400.0) * 34.0
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct AqiDisplayProps {
    pub aqi_result: Option<AqiResult>,
    #[prop_or(false)]
    pub is_loading: bool,
}

#[function_component(AqiDisplay)]
pub fn aqi_display(props: &AqiDisplayProps) -> Html {
    html! {
        <div class="aqi-display">
            {
                if props.is_loading {
                    html! { <div class="aqi-loading">{ "Calculating AQI..." }</div> }
                } else if let Some(aqi_result) = &props.aqi_result {
                    html! {
                        <div class="aqi-content">
                            <div class="aqi-row">
                                <div class="aqi-index-group">
                                    <div class="aqi-index-label">{ "Air Quality Index (AQI):" }</div>
                                    <div class="aqi-value">{ aqi_result.value }</div>
                                </div>

                                <div class="aqi-category">{ &aqi_result.category.name }</div>
                                <div class="aqi-bar-container">
                                    <div class="aqi-bar">
                                        <div class="aqi-bar-segment aqi-good">{ "Good" }</div>
                                        <div class="aqi-bar-segment aqi-moderate">{ "Moderate" }</div>
                                        <div class="aqi-bar-segment aqi-unhealthy">{ "Unhealthy" }</div>
                                    </div>
                                    <div class="aqi-indicator" style={format!("left: {}%", calculate_indicator_position(aqi_result.value))}></div>
                                </div>

                                <div class="aqi-dominant">
                                    <span class="aqi-dominant-label">{ "Dominant Pollutant:" }</span>
                                    <span class="aqi-dominant-value">{ &aqi_result.dominant_pollutant }</span>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! { <div class="aqi-no-data">{ "No data available to calculate AQI" }</div> }
                }
            }
        </div>
    }
}
