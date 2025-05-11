#!/bin/bash

# List of chart files to update
CHART_FILES=(
  "webserver/frontend/src/app/instances/charts/humidity.rs"
  "webserver/frontend/src/app/instances/charts/pressure.rs"
  "webserver/frontend/src/app/instances/charts/carbon_iv_oxide.rs"
  "webserver/frontend/src/app/instances/charts/carbon_ii_oxide.rs"
  "webserver/frontend/src/app/instances/charts/ozone.rs"
)

# Update each chart file
for file in "${CHART_FILES[@]}"; do
  echo "Updating $file..."
  
  # Add location filter import
  sed -i 's/use crate::app::utils::time_filter::{TimeRange, filter_data_by_time_range};/use crate::app::utils::time_filter::{TimeRange, filter_data_by_time_range};\nuse crate::app::utils::location_filter::{LocationFilter, filter_data_by_location};/' "$file"
  
  # Update props struct
  sed -i 's/pub struct \([a-zA-Z]*\)ChartProps {\n    pub time_range: TimeRange,/pub struct \1ChartProps {\n    pub time_range: TimeRange,\n    #[prop_or_else(|| LocationFilter::All)]\n    pub location_filter: LocationFilter,/' "$file"
  
  # Update function component to add location_filter
  sed -i 's/pub fn \([a-zA-Z_]*\)(props: &\([a-zA-Z]*\)ChartProps) -> Html {\n    let chart_config = use_state(|| None::<TimeSeriesChartProps>);\n    let time_range = props.time_range.clone();/pub fn \1(props: \&\2ChartProps) -> Html {\n    let chart_config = use_state(|| None::<TimeSeriesChartProps>);\n    let time_range = props.time_range.clone();\n    let location_filter = props.location_filter.clone();/' "$file"
  
  # Update data fetching to include location filtering
  sed -i 's/{
        let chart_config = chart_config.clone();
        let time_range = time_range.clone();

        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    \/\/ Filter data by time range
                    let filtered_data = filter_data_by_time_range(
                        \&fetched_data,
                        \&time_range,
                        |record| parse_timestamp(\&record.timestamp).ok()
                    );/{
        let chart_config = chart_config.clone();
        let time_range = time_range.clone();
        let location_filter = location_filter.clone();

        spawn_local(async move {
            match get_air_quality_data().await {
                Ok(fetched_data) => {
                    \/\/ First filter by time range
                    let time_filtered_data = filter_data_by_time_range(
                        \&fetched_data,
                        \&time_range,
                        |record| parse_timestamp(\&record.timestamp).ok()
                    );
                    
                    \/\/ Then filter by location
                    let filtered_data = filter_data_by_location(
                        \&time_filtered_data,
                        \&location_filter,
                        |record| record.location.clone()
                    );/' "$file"
  
  # Update effect hook to include location filter changes
  sed -i 's/    \/\/ Re-fetch data when time range changes
    {
        let chart_config = chart_config.clone();
        use_effect_with(time_range, move |_| {
            chart_config.set(None); \/\/ Reset chart to show loading state
            || ()
        });
    }/    \/\/ Re-fetch data when time range or location changes
    {
        let chart_config = chart_config.clone();
        let location_filter_clone = location_filter.clone();
        
        use_effect_with(time_range, move |_| {
            chart_config.set(None); \/\/ Reset chart to show loading state
            || ()
        });
        
        let chart_config = chart_config.clone();
        use_effect_with(location_filter_clone, move |_| {
            chart_config.set(None); \/\/ Reset chart to show loading state
            || ()
        });
    }/' "$file"
  
  echo "Updated $file"
done

echo "All chart files updated!"
