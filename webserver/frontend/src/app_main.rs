use yew::prelude::*;
use crate::app::pages::dashboard::Dashboard;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{ "Air Quality Monitoring" }</h1>
            <Dashboard />
        </div>
    }
}
