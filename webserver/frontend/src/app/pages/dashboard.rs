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
        <div>
            <h2>{ "Air Quality Dashboard" }</h2>
            <ParticulateMatterChart />
            <TemperatureChart />
            <CarbonIVOxideChart />
            <CarbonIIOxideChart />
            <HumidityChart />
            <PressureChart />
            <OzoneChart />
        </div>
    }
}