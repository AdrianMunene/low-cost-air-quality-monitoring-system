// @generated automatically by Diesel CLI.

diesel::table! {
    air_quality_data (id) {
        id -> Integer,
        timestamp -> Timestamp,
        longitude -> Nullable<Double>,
        latitude -> Nullable<Double>,
        temperature -> Nullable<Double>,
        pressure -> Nullable<Double>,
        humidity -> Nullable<Double>,
        pm1_0 -> Nullable<Double>,
        pm2_5 -> Nullable<Double>,
        pm10 -> Nullable<Double>,
        co2 -> Nullable<Double>,
        co -> Nullable<Double>,
        o3 -> Nullable<Double>,
    }
}
