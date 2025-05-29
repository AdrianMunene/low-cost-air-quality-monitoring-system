use crate::sensors::{ sim808::Sim808, serial::Serial };
use esp_hal::{
    gpio::GpioPin,
    peripherals::{ UART0, UART1 }
};

use esp_println::println;

use chrono::{NaiveDateTime, Duration };

use alloc::{string::String, vec::Vec, format};
use alloc::string::ToString;

pub struct Sim808Functions {
    pub sim808: Sim808<'static>,
    pub serial: Serial<'static>,
}

impl Sim808Functions {
    pub fn new(
        uart0: UART0, 
        rx0: GpioPin<17>, 
        tx0: GpioPin<16>, 
        uart1: UART1, 
        rx1: GpioPin<20>, 
        tx1: GpioPin<21>,
    ) -> Self {

        let serial = Serial::new(uart0, rx0, tx0, 9600).unwrap();
        let sim808 = Sim808::new(uart1, rx1, tx1, 9600).unwrap();

        Sim808Functions { sim808, serial }
    }

    pub async fn config_sim808(&mut self) {
        let mut buffer = [0u8; 64];

        self.sim808.send_command("AT+SAPBR=1,1".as_bytes()).await.unwrap();
        
        if let Ok(bytes_read) = self.sim808.read_response(&mut buffer).await {
            if bytes_read > 0 {
                if buffer[..bytes_read] == *"ERROR".as_bytes() {
                    println!("Error sending AT+SAPBR=1,1 command")
                } else {
                    println!("SIM808 responded with: {:?}", buffer);
                    self.serial.send_response(&buffer).await.unwrap();
                }
                buffer.fill(0);
            }
        }

        self.sim808.send_command("AT+CGNSPWR=1".as_bytes()).await.unwrap();

        if let Ok(bytes_read) = self.sim808.read_response(&mut buffer).await {
            if bytes_read > 0 {
                if buffer[..bytes_read] == *"ERROR".as_bytes() {
                    println!("Error sending AT+CGNSPWR=1 command")
                } else {
                    println!("SIM808 responded with: {:?}", buffer);
                    self.serial.send_response(&buffer).await.unwrap();
                }
                buffer.fill(0);
            }
        }
    }

    pub async fn get_location_timestamp(&mut self) -> Option<(f64, f64, String)> {
        self.sim808.send_command("AT+CGNSINF".as_bytes()).await.ok()?;
        
        let mut buffer = [0u8; 128];

        let bytes_read = self.sim808.read_response(&mut buffer).await.ok()?;

        let response = core::str::from_utf8(&buffer[..bytes_read]).ok()?;

        let line = response.trim();

        if let Some(start) = line.find("+CGNSINF:") {
            let fields: Vec<&str> = line[start..].split(',').collect();

            if fields.len() >= 5 && fields[1] == "1" {
                let timestamp_raw = fields[2];

                let latitude = fields[3].parse::<f64>().ok()?;
                let longitude = fields[4].parse::<f64>().ok()?;
                let utc_timestamp = format!(
                    "{}-{}-{}T{}:{}:{}Z",
                    &timestamp_raw[0..4],
                    &timestamp_raw[4..6],
                    &timestamp_raw[6..8],
                    &timestamp_raw[8..10],
                    &timestamp_raw[10..12],
                    &timestamp_raw[12..14]
                );

                let utc_naive = NaiveDateTime::parse_from_str(&utc_timestamp, "%Y-%m-%dT%H:%M:%SZ").ok()?;

                let local_time = utc_naive + Duration::hours(3); 

                let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

                return Some((latitude, longitude, formatted_time));
            } else {
                return None
            }
        } else {
            return None
        }
 
    }

    pub async fn send_to_webserver(&mut self, url: &str, json_payload: &str) {
        let mut buffer = [0u8; 128];

        // Ensure GPRS context is open
        self.sim808.send_command(b"AT+SAPBR=3,1,\"Contype\",\"GPRS\"").await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok();
        buffer.fill(0);

        self.sim808.send_command(b"AT+SAPBR=3,1,\"APN\",\"safaricom\"").await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok();
        buffer.fill(0);

        self.sim808.send_command(b"AT+SAPBR=1,1").await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok();
        buffer.fill(0);

        self.sim808.send_command(b"AT+SAPBR=2,1").await.unwrap(); // Query bearer status
        self.sim808.read_response(&mut buffer).await.ok();
        buffer.fill(0);

        // Start HTTP service
        self.sim808.send_command(b"AT+HTTPINIT").await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok();
        buffer.fill(0);

        // Set HTTP parameters
        let url_cmd = format!("AT+HTTPPARA=\"URL\",\"{}\"", url);
        self.sim808.send_command(url_cmd.as_bytes()).await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok();
        buffer.fill(0);

        self.sim808.send_command(b"AT+HTTPPARA=\"CONTENT\",\"application/json\"").await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok();
        buffer.fill(0);

        // Provide data length
        let data_len_cmd = format!("AT+HTTPDATA={},10000", json_payload.len());
        self.sim808.send_command(data_len_cmd.as_bytes()).await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok(); // Wait for "DOWNLOAD"
        buffer.fill(0);

        // Send the actual payload
        self.sim808.send_command(json_payload.as_bytes()).await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok(); // Wait for "OK"
        buffer.fill(0);

        // Start POST
        self.sim808.send_command(b"AT+HTTPACTION=1").await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok(); // Wait for status like "+HTTPACTION: 1,200,xxx"
        buffer.fill(0);

        // Read server response (optional)
        self.sim808.send_command(b"AT+HTTPREAD").await.unwrap();
        if let Ok(read_bytes) = self.sim808.read_response(&mut buffer).await {
            if read_bytes > 0 {
                println!("Server response: {:?}", &buffer[..read_bytes]);
            }
        }   
        buffer.fill(0);

        // End HTTP session
        self.sim808.send_command(b"AT+HTTPTERM").await.unwrap();
        self.sim808.read_response(&mut buffer).await.ok();
    }

    pub async fn send_data(&mut self, temperature: f32, pressure: f32, humidity: f32, pm1_0: u16, pm2_5: u16, pm10: u16, co2: u16, co: u16) {
        if let Some((latitude, longitude, timestamp)) = self.get_location_timestamp().await {
            let payload = format!(
                r#"{{
                    "timestamp": "{}",
                    "latitude": {:.6},
                    "longitude": {:.6},
                    "temperature": {:.2},
                    "pressure": {:.2},
                    "humidity": {:.2},
                    "pm1_0": {},
                    "pm2_5": {},
                    "pm10": {},
                    "co2": {},
                    "co": {},
                    "o3": 0
                }}"#,
                timestamp, latitude, longitude,
                temperature, pressure, humidity,
                pm1_0, pm2_5, pm10, co2, co
            );

        // Send the payload to the server
            self.send_to_webserver("https://airqualitymonitoring.cc/airquality", &payload).await;
        } else {
            println!("Failed to get location or timestamp");
        }
    }


}