use crate::sensors::{ sim808::Sim808, serial::Serial };
use esp_hal::{
    gpio::GpioPin,
    peripherals::{ UART0, UART1 }
};

use esp_println::println;

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

    //pub fn get_location_timestamp

    //pub send_to_webserver
}