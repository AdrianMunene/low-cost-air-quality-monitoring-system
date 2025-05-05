use chrono::NaiveDateTime;
use crate::models::air_quality::AirQualityInputOutput;

pub fn validate_air_quality_data(data: &AirQualityInputOutput) -> Result<(), String> {
    // Validate timestamp format
    match NaiveDateTime::parse_from_str(&data.timestamp, "%Y-%m-%d %H:%M:%S") {
        Ok(_) => (),
        Err(e) => return Err(format!("Invalid timestamp format: {}", e)),
    }
    
    // Validate coordinates if provided
    if let Some(longitude) = data.longitude {
        if longitude < -180.0 || longitude > 180.0 {
            return Err("Longitude must be between -180 and 180".to_string());
        }
    }
    
    if let Some(latitude) = data.latitude {
        if latitude < -90.0 || latitude > 90.0 {
            return Err("Latitude must be between -90 and 90".to_string());
        }
    }
    
    // Validate sensor readings if provided
    if let Some(temperature) = data.temperature {
        if temperature < -100.0 || temperature > 100.0 {
            return Err("Temperature out of reasonable range".to_string());
        }
    }
    
    if let Some(pressure) = data.pressure {
        if pressure < 800.0 || pressure > 1200.0 {
            return Err("Pressure out of reasonable range".to_string());
        }
    }
    
    if let Some(humidity) = data.humidity {
        if humidity < 0.0 || humidity > 100.0 {
            return Err("Humidity must be between 0 and 100".to_string());
        }
    }
    
    // Validate particulate matter readings
    if let Some(pm1_0) = data.pm1_0 {
        if pm1_0 < 0.0 || pm1_0 > 1000.0 {
            return Err("PM1.0 out of reasonable range".to_string());
        }
    }
    
    if let Some(pm2_5) = data.pm2_5 {
        if pm2_5 < 0.0 || pm2_5 > 1000.0 {
            return Err("PM2.5 out of reasonable range".to_string());
        }
    }
    
    if let Some(pm10) = data.pm10 {
        if pm10 < 0.0 || pm10 > 1000.0 {
            return Err("PM10 out of reasonable range".to_string());
        }
    }
    
    // Validate gas readings
    if let Some(co2) = data.co2 {
        if co2 < 0.0 || co2 > 10000.0 {
            return Err("CO2 out of reasonable range".to_string());
        }
    }
    
    if let Some(co) = data.co {
        if co < 0.0 || co > 1000.0 {
            return Err("CO out of reasonable range".to_string());
        }
    }
    
    if let Some(o3) = data.o3 {
        if o3 < 0.0 || o3 > 1000.0 {
            return Err("O3 out of reasonable range".to_string());
        }
    }
    
    Ok(())
}
