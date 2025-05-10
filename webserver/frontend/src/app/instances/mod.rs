// Chart instances
pub mod charts;

// Average metrics instances
pub mod average_metrics;

// AQI metrics instances
pub mod aqi;

// Re-export for backward compatibility
pub use charts::particulate_matter;
pub use charts::carbon_ii_oxide;
pub use charts::ozone;
pub use charts::temperature;
pub use charts::pressure;
pub use charts::humidity;
pub use charts::carbon_iv_oxide;

pub use average_metrics::average_environmental;
pub use average_metrics::average_particulate;
pub use average_metrics::average_co;
pub use average_metrics::average_co2;
pub use average_metrics::average_o3;

pub use aqi::aqi_metrics;