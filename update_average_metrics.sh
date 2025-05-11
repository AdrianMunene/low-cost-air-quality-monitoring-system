#!/bin/bash

# List of average metrics files to update
METRICS_FILES=(
  "webserver/frontend/src/app/instances/average_metrics/average_particulate.rs"
  "webserver/frontend/src/app/instances/average_metrics/average_co.rs"
  "webserver/frontend/src/app/instances/average_metrics/average_co2.rs"
  "webserver/frontend/src/app/instances/average_metrics/average_o3.rs"
)

# Update each metrics file
for file in "${METRICS_FILES[@]}"; do
  echo "Updating $file..."
  
  # Add location filter import
  sed -i 's/use crate::app::utils::time_filter::TimeRange;/use crate::app::utils::time_filter::TimeRange;\nuse crate::app::utils::location_filter::{LocationFilter, filter_data_by_location};/' "$file"
  
  # Update props struct
  sed -i 's/#\[derive(Properties, Clone, PartialEq)\].*pub struct \([a-zA-Z]*\)MetricsProps {.*pub time_range: TimeRange,.*}/#\[derive(Properties, Clone, PartialEq)\]\npub struct \1MetricsProps {\n    pub time_range: TimeRange,\n    #\[prop_or_else(|| LocationFilter::All)\]\n    pub location_filter: LocationFilter,\n}/' "$file"
  
  # Update function component to add location_filter
  sed -i 's/#\[function_component(Average\([a-zA-Z]*\)Metrics)\].*pub fn average_\([a-zA-Z_]*\)(props: &\([a-zA-Z]*\)MetricsProps) -> Html {.*let metrics = use_state(|| Vec::<MetricData>::new());.*let is_loading = use_state(|| true);.*let time_range = props.time_range.clone();/#\[function_component(Average\1Metrics)\]\npub fn average_\2(props: \&\3MetricsProps) -> Html {\n    let metrics = use_state(|| Vec::<MetricData>::new());\n    let is_loading = use_state(|| true);\n    let time_range = props.time_range.clone();\n    let location_filter = props.location_filter.clone();/' "$file"
  
  # Update effect hook to include location filter
  sed -i 's/    \/\/ Fetch data and calculate averages.*{.*let metrics = metrics.clone();.*let is_loading = is_loading.clone();.*.*use_effect_with(time_range.clone(), move |time_range| {.*let time_range = time_range.clone();.*is_loading.set(true);.*metrics.set(Vec::new());/    \/\/ Fetch data and calculate averages\n    {\n        let metrics = metrics.clone();\n        let is_loading = is_loading.clone();\n        let location_filter = location_filter.clone();\n\n        use_effect_with((time_range.clone(), location_filter.clone()), move |(time_range, location_filter)| {\n            let time_range = time_range.clone();\n            let location_filter = location_filter.clone();\n            is_loading.set(true);\n            metrics.set(Vec::new());/' "$file"
  
  # Update data fetching to include location filtering
  sed -i 's/                match get_air_quality_data().await {.*Ok(data) => {.*let mut metrics_vec = Vec::new();/                match get_air_quality_data().await {\n                    Ok(fetched_data) => {\n                        \/\/ Filter data by location\n                        let data = filter_data_by_location(\n                            \&fetched_data,\n                            \&location_filter,\n                            |record| record.location.clone()\n                        );\n                        \n                        let mut metrics_vec = Vec::new();/' "$file"
  
  echo "Updated $file"
done

echo "All average metrics files updated!"
