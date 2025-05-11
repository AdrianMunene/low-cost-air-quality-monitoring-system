#!/bin/bash

# List of metrics files to update
METRICS_FILES=(
  "webserver/frontend/src/app/instances/average_metrics/average_particulate.rs"
  "webserver/frontend/src/app/instances/average_metrics/average_co.rs"
  "webserver/frontend/src/app/instances/average_metrics/average_co2.rs"
  "webserver/frontend/src/app/instances/average_metrics/average_o3.rs"
)

# Update each metrics file
for file in "${METRICS_FILES[@]}"; do
  echo "Updating $file..."
  
  # Update props struct
  sed -i 's/#\[derive(Properties, Clone, PartialEq)\].*pub struct \([a-zA-Z]*\)MetricsProps {.*pub time_range: TimeRange,.*}/#\[derive(Properties, Clone, PartialEq)\]\npub struct \1MetricsProps {\n    pub time_range: TimeRange,\n    #\[prop_or_else(|| LocationFilter::All)\]\n    pub location_filter: LocationFilter,\n}/' "$file"
  
  echo "Updated $file"
done

echo "All metrics files updated!"
