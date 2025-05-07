use yew::prelude::*;
use crate::app::instances::particulate_matter::ParticulateMatterChart;
use crate::app::instances::temperature::TemperatureChart;
use crate::app::instances::pressure::PressureChart;
use crate::app::instances::humidity::HumidityChart;
use crate::app::instances::carbon_iv_oxide::CarbonIVOxideChart;
use crate::app::instances::carbon_ii_oxide::CarbonIIOxideChart;
use crate::app::instances::ozone::OzoneChart;
use crate::app::components::time_filter::TimeFilterComponent;
use crate::app::utils::time_filter::TimeRange;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    // State for the selected time range - default to LastMonth
    let selected_time_range = use_state(|| TimeRange::LastMonth);

    // Callback for when the time range changes
    let on_time_range_change = {
        let selected_time_range = selected_time_range.clone();
        Callback::from(move |new_range: TimeRange| {
            log::info!("Time range changed to: {:?}", new_range);
            selected_time_range.set(new_range);
        })
    };

    html! {
        <div class="dashboard-wrapper">
            // Time filter component
            <div class="dashboard-controls">
                <TimeFilterComponent
                    selected_range={(*selected_time_range).clone()}
                    on_range_change={on_time_range_change.clone()}
                />
            </div>

            // Main grid with all charts
            <div class="dashboard-grid">
                // PM Chart spans full width at the top
                <div class="chart-container grid-item-1-1 chart-large">
                    <div class="chart-header">
                        <h3>{ "Particulate Matter" }</h3>
                        <span class="chart-subtitle">{ "µg/m³" }</span>
                    </div>
                    <div class="chart-content">
                        <ParticulateMatterChart time_range={(*selected_time_range).clone()} />
                    </div>
                </div>


                // Temperature chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Temperature" }</h3>
                        <span class="chart-subtitle">{ "°C" }</span>
                    </div>
                    <div class="chart-content">
                        <TemperatureChart time_range={(*selected_time_range).clone()} />
                    </div>
                </div>

                // Humidity chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Humidity" }</h3>
                        <span class="chart-subtitle">{ "%" }</span>
                    </div>
                    <div class="chart-content">
                        <HumidityChart time_range={(*selected_time_range).clone()} />
                    </div>
                </div>

                // Pressure chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Pressure" }</h3>
                        <span class="chart-subtitle">{ "hPa" }</span>
                    </div>
                    <div class="chart-content">
                        <PressureChart time_range={(*selected_time_range).clone()} />
                    </div>
                </div>

                // CO2 chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Carbon (IV) Oxide" }</h3>
                        <span class="chart-subtitle">{ "CO₂ (ppm)" }</span>
                    </div>
                    <div class="chart-content">
                        <CarbonIVOxideChart time_range={(*selected_time_range).clone()} />
                    </div>
                </div>

                // CO chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Carbon (II) Oxide" }</h3>
                        <span class="chart-subtitle">{ "CO (ppm)" }</span>
                    </div>
                    <div class="chart-content">
                        <CarbonIIOxideChart time_range={(*selected_time_range).clone()} />
                    </div>
                </div>

                // Ozone chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Ozone" }</h3>
                        <span class="chart-subtitle">{ "O₃ (ppb)" }</span>
                    </div>
                    <div class="chart-content">
                        <OzoneChart time_range={(*selected_time_range).clone()} />
                    </div>
                </div>
            </div>
        </div>
    }
}
