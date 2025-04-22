use yew::prelude::*;
use crate::app::instances::particulate_matter::ParticulateMatterChart;
use crate::app::instances::temperature::TemperatureChart;
use crate::app::instances::carbon_iv_oxide::CarbonIVOxideChart;
use crate::app::instances::carbon_ii_oxide::CarbonIIOxideChart;
use crate::app::instances::humidity::HumidityChart;
use crate::app::instances::pressure::PressureChart;
use crate::app::instances::ozone::OzoneChart;
use gloo_timers::callback::Interval;

// Minimum time between API requests (in milliseconds)
const THROTTLE_MS: u32 = 10000; // 10 seconds

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    // Set up a periodic refresh
    let refresh_counter = use_state(|| 0);

    // Set up an interval to trigger refreshes
    {
        let refresh_counter = refresh_counter.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(THROTTLE_MS, move || {
                refresh_counter.set(*refresh_counter + 1);
            });

            move || drop(interval)
        });
    }

    // Force a re-render when the counter changes
    let _ = *refresh_counter;

    html! {
        <div class="dashboard-wrapper">
            <h2 class="dashboard-title">{ "Air Quality Dashboard" }</h2>

            // Main grid with all charts
            <div class="dashboard-grid">
                // PM Chart spans full width at the top
                <div class="chart-container grid-item-1-1 chart-large">
                    <div class="chart-header">
                        <h3>{ "Particulate Matter" }</h3>
                        <span class="chart-subtitle">{ "µg/m³" }</span>
                    </div>
                    <div class="chart-content">
                        <ParticulateMatterChart />
                    </div>
                </div>

                // Temperature chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Temperature" }</h3>
                        <span class="chart-subtitle">{ "°C" }</span>
                    </div>
                    <div class="chart-content">
                        <TemperatureChart />
                    </div>
                </div>

                // Humidity chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Humidity" }</h3>
                        <span class="chart-subtitle">{ "%" }</span>
                    </div>
                    <div class="chart-content">
                        <HumidityChart />
                    </div>
                </div>

                // Pressure chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Pressure" }</h3>
                        <span class="chart-subtitle">{ "hPa" }</span>
                    </div>
                    <div class="chart-content">
                        <PressureChart />
                    </div>
                </div>

                // CO2 chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Carbon Dioxide" }</h3>
                        <span class="chart-subtitle">{ "CO₂ (ppm)" }</span>
                    </div>
                    <div class="chart-content">
                        <CarbonIVOxideChart />
                    </div>
                </div>

                // CO chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Carbon Monoxide" }</h3>
                        <span class="chart-subtitle">{ "CO (ppm)" }</span>
                    </div>
                    <div class="chart-content">
                        <CarbonIIOxideChart />
                    </div>
                </div>

                // Ozone chart
                <div class="chart-container chart-medium">
                    <div class="chart-header">
                        <h3>{ "Ozone" }</h3>
                        <span class="chart-subtitle">{ "O₃ (ppb)" }</span>
                    </div>
                    <div class="chart-content">
                        <OzoneChart />
                    </div>
                </div>
            </div>
        </div>
    }
}
