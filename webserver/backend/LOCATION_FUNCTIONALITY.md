# Location Functionality in Air Quality Monitoring System

## Overview

The Air Quality Monitoring System now includes automatic location determination based on latitude and longitude coordinates. This document explains how the location functionality works and how to use it.

## How It Works

1. When air quality data is submitted to the API, the system expects latitude and longitude coordinates.
2. The backend automatically performs reverse geocoding using the Nominatim API (OpenStreetMap) to convert these coordinates into a human-readable location string.
3. The location string is stored in the database along with the air quality data.
4. Any location string provided in the input data is ignored - only the geocoded location from coordinates is used.

## API Usage

### Submitting Air Quality Data

When submitting air quality data to the `/airquality` endpoint, include the latitude and longitude coordinates:

```json
{
  "timestamp": "2025-03-30 12:34:56",
  "latitude": 37.7749,
  "longitude": -122.4194,
  "temperature": 18.5,
  "pressure": 1012.3,
  "humidity": 60.2,
  "pm1_0": 5.1,
  "pm2_5": 10.2,
  "pm10": 20.5,
  "co2": 400.0,
  "co": 0.5,
  "o3": 0.03
}
```

Note that:
- The `location` field should not be included in the input data.
- If a `location` field is included, it will be ignored.
- If latitude or longitude is missing, no location will be determined.

### Retrieving Air Quality Data

When retrieving data from the `/airquality` endpoint, the response will include the geocoded location:

```json
[
  {
    "timestamp": "2025-03-30 12:34:56",
    "latitude": 37.7749,
    "longitude": -122.4194,
    "location": "San Francisco, California, USA",
    "temperature": 18.5,
    "pressure": 1012.3,
    "humidity": 60.2,
    "pm1_0": 5.1,
    "pm2_5": 10.2,
    "pm10": 20.5,
    "co2": 400.0,
    "co": 0.5,
    "o3": 0.03
  }
]
```

## Filtering by Location

The frontend includes a location filter component that allows users to filter air quality data by location. This filter works with the geocoded locations determined by the backend.

## Implementation Details

The location functionality is implemented in the following files:

- `webserver/backend/src/geocoding.rs`: Contains the reverse geocoding functionality.
- `webserver/backend/src/handlers.rs`: Contains the API handlers that use the geocoding functionality.
- `webserver/frontend/src/app/utils/location_filter.rs`: Contains the location filtering functionality.
- `webserver/frontend/src/app/components/location_filter.rs`: Contains the location filter UI component.

## Testing

Unit tests for the location functionality are included in:

- `webserver/backend/src/handlers.rs`: Tests for the `get_location_from_coordinates` function.
- `webserver/backend/tests/api_tests.rs`: Integration tests for the location functionality.

To run the unit tests:

```bash
cd webserver/backend
cargo test
```

To run the integration tests, first start the backend server:

```bash
cd webserver/backend
cargo run
```

Then in another terminal:

```bash
cd webserver/backend
cargo test --test api_tests
```
