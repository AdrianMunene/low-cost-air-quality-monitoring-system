use yew::prelude::*;
use crate::app::components::time_filter::TimeFilterComponent;
use crate::app::utils::time_filter::TimeRange;

// Import chart components
use crate::app::instances::charts::particulate_matter::ParticulateMatterChart;
use crate::app::instances::charts::temperature::TemperatureChart;
use crate::app::instances::charts::pressure::PressureChart;
use crate::app::instances::charts::humidity::HumidityChart;
use crate::app::instances::charts::carbon_iv_oxide::CarbonIVOxideChart;
use crate::app::instances::charts::carbon_ii_oxide::CarbonIIOxideChart;
use crate::app::instances::charts::ozone::OzoneChart;

// Import average metrics components
use crate::app::instances::average_metrics::average_environmental::AverageEnvironmentalMetrics;
use crate::app::instances::average_metrics::average_particulate::AverageParticulateMetrics;
use crate::app::instances::average_metrics::average_co::AverageCOMetrics;
use crate::app::instances::average_metrics::average_co2::AverageCO2Metrics;
use crate::app::instances::average_metrics::average_o3::AverageO3Metrics;

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

            // Average metrics row
            <div class="dashboard-metrics">
                // Environmental metrics (Temperature, Humidity, Pressure)
                <AverageEnvironmentalMetrics time_range={(*selected_time_range).clone()} />

                // Particulate matter metrics (PM1.0, PM2.5, PM10)
                <AverageParticulateMetrics time_range={(*selected_time_range).clone()} />

                // Carbon monoxide metrics
                <AverageCOMetrics time_range={(*selected_time_range).clone()} />

                // Carbon dioxide metrics
                <AverageCO2Metrics time_range={(*selected_time_range).clone()} />

                // Ozone metrics
                <AverageO3Metrics time_range={(*selected_time_range).clone()} />
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
