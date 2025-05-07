use yew::prelude::*;
use crate::app::pages::dashboard::Dashboard;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app-container">
            <header class="app-header">
                <div class="logo">
                    <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
                        <path d="M12,20A6,6 0 0,1 6,14C6,10 12,3.25 12,3.25C12,3.25 18,10 18,14A6,6 0 0,1 12,20Z" />
                    </svg>
                </div>
                <h1>{ "Air Quality Monitoring System Dashboard" }</h1>
                <div class="header-actions">
                    <span class="status-indicator online">{ "Live Data" }</span>
                </div>
            </header>
            <main class="app-content">
                <Dashboard />
            </main>
            <footer class="app-footer">
                <p>{ "Low-Cost Air Quality Monitoring System" }</p>
            </footer>
        </div>
    }
}
