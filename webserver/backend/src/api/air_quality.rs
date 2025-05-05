use axum::{Extension, Json};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json::json;
use tracing::info;

use crate::db::DatabasePool;
use crate::dto::AirQualityInputOutput;
use crate::error::ApiError;
use crate::utils::validate_air_quality_data;
use database::models::{AirQualityData, NewAirQualityData};
use database::schema::air_quality_data::dsl::air_quality_data;

pub async fn create_air_quality_record(
    Extension(pool): Extension<DatabasePool>,
    Json(input): Json<AirQualityInputOutput>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate input data
    validate_air_quality_data(&input)
        .map_err(|e| ApiError::ValidationError(e))?;

    let mut conn = pool.get()?;

    let timestamp = NaiveDateTime::parse_from_str(&input.timestamp, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| ApiError::ValidationError(format!("Invalid timestamp: {}", e)))?;

    let new_record = NewAirQualityData {
        timestamp,
        longitude: input.longitude,
        latitude: input.latitude,
        temperature: input.temperature,
        pressure: input.pressure,
        humidity: input.humidity,
        pm1_0: input.pm1_0,
        pm2_5: input.pm2_5,
        pm10: input.pm10,
        co2: input.co2,
        co: input.co,
        o3: input.o3,
    };

    diesel::insert_into(air_quality_data)
        .values(&new_record)
        .execute(&mut conn)?;

    Ok(Json(json!({ "status": "success" })))
}

pub async fn get_air_quality_record(
    Extension(pool): Extension<DatabasePool>
) -> Result<Json<Vec<AirQualityInputOutput>>, ApiError> {
    info!("Received request for air quality data");

    let mut conn = pool.get()?;

    let records = air_quality_data.load::<AirQualityData>(&mut conn)?;
    info!("Successfully loaded {} air quality records", records.len());

    let output: Vec<AirQualityInputOutput> = records.into_iter().map(|record| {
        AirQualityInputOutput {
            timestamp: record.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            longitude: record.longitude,
            latitude: record.latitude,
            temperature: record.temperature,
            pressure: record.pressure,
            humidity: record.humidity,
            pm1_0: record.pm1_0,
            pm2_5: record.pm2_5,
            pm10: record.pm10,
            co2: record.co2,
            co: record.co,
            o3: record.o3,
        }
    }).collect();

    Ok(Json(output))
}
