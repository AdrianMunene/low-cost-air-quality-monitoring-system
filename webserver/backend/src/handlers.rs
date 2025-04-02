use serde::{ Serialize, Deserialize };
use axum::{ Extension, Json };
use chrono::NaiveDateTime;
use serde_json::json;
use diesel::prelude::*;
use database::models::{AirQualityData, NewAirQualityData};
use database::schema::air_quality_data::dsl::air_quality_data;
use crate::database::DatabasePool;


#[derive(Debug, Serialize, Deserialize)]
pub struct AirQualityInputOutput {
    pub timestamp: String,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub temperature: Option<f64>,
    pub pressure: Option<f64>,
    pub humidity: Option<f64>,
    pub pm1_0: Option<f64>,
    pub pm2_5: Option<f64>,
    pub pm10: Option<f64>,
    pub co2: Option<f64>,
    pub co: Option<f64>,
    pub o3: Option<f64>

}

pub async fn create_air_quality_record(
    Extension(pool): Extension<DatabasePool>, 
    Json(input): Json<AirQualityInputOutput>,
) -> Result<Json<serde_json::Value>, String> {

    let mut conn = pool.get().map_err(|e| e.to_string())?;

    let timestamp = NaiveDateTime::parse_from_str(&input.timestamp, "%Y-%m-%d %H:%M:%S")
    .map_err(|e| format!("Invalid timestamp: {}", e))?;

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

    diesel::insert_into(air_quality_data).values(&new_record).execute(&mut conn).map_err(|e| e.to_string())?;

    Ok(Json(json!({ "status": "success" })))
}

pub async fn get_air_quality_record(
    Extension(pool): Extension<DatabasePool>
) -> Result<Json<Vec<AirQualityInputOutput>>, String> {
    
    let mut conn = pool.get().map_err(|e| e.to_string())?;

    let records = air_quality_data.load::<AirQualityData>(&mut conn).map_err(|e| e.to_string())?;

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