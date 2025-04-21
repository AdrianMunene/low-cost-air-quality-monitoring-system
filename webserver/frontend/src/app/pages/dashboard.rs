use yew::prelude::*;
use crate::app::instances::particulate_matter::ParticulateMatterChart;
use crate::app::instances::temperature::TemperatureChart;
use crate::app::instances::carbon_iv_oxide::CarbonIVOxideChart;
use crate::app::instances::carbon_ii_oxide::CarbonIIOxideChart;
use crate::app::instances::humidity::HumidityChart;
use crate::app::instances::pressure::PressureChart;
use crate::app::instances::ozone::OzoneChart;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="dashboard-wrapper">
            <h2 class="dashboard-title">{ "Air Quality Dashboard" }</h2>
            
            // PM Chart in its own row
            <div class="pm-chart-container">
                <div class="chart-container">
                    <ParticulateMatterChart />
                </div>
            </div>

            // Grid for other charts
            <div class="dashboard-grid">
                <div class="chart-container">
                    <TemperatureChart />
                </div>
                <div class="chart-container">
                    <HumidityChart />
                </div>
                <div class="chart-container">
                    <PressureChart />
                </div>
                <div class="chart-container">
                    <CarbonIVOxideChart />
                </div>
                <div class="chart-container">
                    <CarbonIIOxideChart />
                </div>
                <div class="chart-container">
                    <OzoneChart />
                </div>
            </div>
        </div>
    }
}
