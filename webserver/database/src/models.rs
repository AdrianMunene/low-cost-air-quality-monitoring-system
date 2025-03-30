use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use crate::schema::air_quality_data;

#[derive(Queryable, Selectable)]
#[diesel(table_name = air_quality_data)]
#[diesel(check_for_backend(Sqlite))]
pub struct AirQualityData {
    pub id: i32,
    pub timestamp: NaiveDateTime,
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

#[derive(Insertable)]
#[diesel(table_name = air_quality_data)]
pub struct NewAirQualityData {
    pub timestamp: NaiveDateTime,
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