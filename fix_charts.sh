#!/bin/bash

# List of chart files to update
CHART_FILES=(
  "webserver/frontend/src/app/instances/charts/pressure.rs"
  "webserver/frontend/src/app/instances/charts/carbon_iv_oxide.rs"
  "webserver/frontend/src/app/instances/charts/carbon_ii_oxide.rs"
  "webserver/frontend/src/app/instances/charts/ozone.rs"
)

# Update each chart file
for file in "${CHART_FILES[@]}"; do
  echo "Updating $file..."
  
  # Update props struct
  sed -i 's/#\[derive(Properties, Clone, PartialEq)\].*pub struct \([a-zA-Z]*\)ChartProps {.*pub time_range: TimeRange,.*}/#\[derive(Properties, Clone, PartialEq)\]\npub struct \1ChartProps {\n    pub time_range: TimeRange,\n    #\[prop_or_else(|| LocationFilter::All)\]\n    pub location_filter: LocationFilter,\n}/' "$file"
  
  # Update function component to add location_filter
  sed -i 's/#\[function_component(\([a-zA-Z]*\)Chart)\].*pub fn \([a-zA-Z_]*\)_chart(props: &\([a-zA-Z]*\)ChartProps) -> Html {.*let chart_config = use_state(|| None::<TimeSeriesChartProps>);.*let time_range = props.time_range.clone();/#\[function_component(\1Chart)\]\npub fn \2_chart(props: \&\3ChartProps) -> Html {\n    let chart_config = use_state(|| None::<TimeSeriesChartProps>);\n    let time_range = props.time_range.clone();\n    let location_filter = props.location_filter.clone();/' "$file"
  
  # Update data fetching to include location filtering
  sed -i 's/    {.*let chart_config = chart_config.clone();.*let time_range = time_range.clone();.*.*spawn_local(async move {.*match get_air_quality_data().await {.*Ok(fetched_data) => {.*\/\/ Filter data by time range.*let filtered_data = filter_data_by_time_range(.*\&fetched_data,.*\&time_range,.*|record| parse_timestamp(\&record.timestamp).ok().*);/    {\n        let chart_config = chart_config.clone();\n        let time_range = time_range.clone();\n        let location_filter = location_filter.clone();\n\n        spawn_local(async move {\n            match get_air_quality_data().await {\n                Ok(fetched_data) => {\n                    \/\/ First filter by time range\n                    let time_filtered_data = filter_data_by_time_range(\n                        \&fetched_data,\n                        \&time_range,\n                        |record| parse_timestamp(\&record.timestamp).ok()\n                    );\n                    \n                    \/\/ Then filter by location\n                    let filtered_data = filter_data_by_location(\n                        \&time_filtered_data,\n                        \&location_filter,\n                        |record| record.location.clone()\n                    );/' "$file"
  
  # Update effect hook to include location filter changes
  sed -i 's/    \/\/ Re-fetch data when time range changes.*{.*let chart_config = chart_config.clone();.*use_effect_with(time_range, move |_| {.*chart_config.set(None); \/\/ Reset chart to show loading state.*|| ().*});.*}/    \/\/ Re-fetch data when time range or location changes\n    {\n        let chart_config_time = chart_config.clone();\n        use_effect_with(time_range, move |_| {\n            chart_config_time.set(None); \/\/ Reset chart to show loading state\n            || ()\n        });\n        \n        let chart_config_location = chart_config.clone();\n        use_effect_with(location_filter, move |_| {\n            chart_config_location.set(None); \/\/ Reset chart to show loading state\n            || ()\n        });\n    }/' "$file"
  
  echo "Updated $file"
done

echo "All chart files updated!"
