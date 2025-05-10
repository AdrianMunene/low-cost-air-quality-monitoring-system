use yew::prelude::*;
use std::collections::HashMap;

/// A single metric to display
#[derive(Clone, PartialEq)]
pub struct MetricData {
    pub label: String,
    pub value: f64,
    pub unit: String,
}

/// Props for the AverageMetrics component
#[derive(Properties, Clone, PartialEq)]
pub struct AverageMetricsProps {
    pub title: String,
    pub metrics: Vec<MetricData>,
    #[prop_or(false)]
    pub is_loading: bool,
}

#[function_component(AverageMetrics)]
pub fn average_metrics(props: &AverageMetricsProps) -> Html {
    html! {
        <div class="average-metrics">
            <div class="metrics-header">
                <h3>{ &props.title }</h3>
            </div>
            <div class="metrics-content">
                {
                    if props.is_loading {
                        html! { <div class="metrics-loading">{ "Loading data..." }</div> }
                    } else if props.metrics.is_empty() {
                        html! { <div class="metrics-empty">{ "No data available" }</div> }
                    } else {
                        html! {
                            <div class="metrics-grid">
                                {
                                    props.metrics.iter().map(|metric| {
                                        html! {
                                            <div class="metric-item" key={metric.label.clone()}>
                                                <div class="metric-label">{ &metric.label }</div>
                                                <div class="metric-value">
                                                    { format!("{:.1}", metric.value) }
                                                    <span class="metric-unit">{ &metric.unit }</span>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    }
                }
            </div>
        </div>
    }
}
