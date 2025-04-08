use yew::prelude::*;
use crate::app::instances::particulate_matter::ParticulateMatterChart;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div>
            <h2>{ "Air Quality Dashboard" }</h2>
            <ParticulateMatterChart />
            // You can add additional instances here.
        </div>
    }
}
