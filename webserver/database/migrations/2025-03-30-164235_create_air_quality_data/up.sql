-- Your SQL goes here
CREATE TABLE air_quality_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    timestamp DATETIME NOT NULL,
    longitude DOUBLE,
    latitude DOUBLE,
    temperature DOUBLE,
    pressure DOUBLE,
    humidity DOUBLE,
    pm1_0 DOUBLE,
    pm2_5 DOUBLE,
    pm10 DOUBLE,
    co2 DOUBLE,
    co DOUBLE,
    o3 DOUBLE
);
