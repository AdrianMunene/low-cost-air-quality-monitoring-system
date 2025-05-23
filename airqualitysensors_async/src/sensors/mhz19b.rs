use crate::communicationprotocols::uart::UartHandler;

use esp_hal::{
    gpio::interconnect::{ PeripheralOutput, PeripheralInput },
    uart::{ Instance, Error },
    peripheral::Peripheral,
};

use core::result::Result;

pub struct Mhz19b<'d> {
    uart_handler: UartHandler<'d>,
}

impl<'d> Mhz19b<'d> {
    pub fn new(
        uart:impl Peripheral<P = impl Instance> + 'd,
        rx:impl Peripheral<P = impl PeripheralInput> + 'd,
        tx: impl Peripheral<P = impl PeripheralOutput> + 'd,
        baudrate: u32,
    )  -> Result<Self, Error> {
        let uart_handler = UartHandler::new(uart, rx, tx, baudrate).unwrap();

        Result::Ok(Self { uart_handler })
    }
    

    pub async fn read_co2(&mut self) -> Result<u16, &'static str> {
        let read_command = [0xFF, 0x01, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79];

        let mut buffer = [0u8;9];

        self.uart_handler.write(&read_command).await.map_err(|_| "Write failed")?;
        self.uart_handler.read(&mut buffer).await.map_err(|_| "Read failed")?;
        self.uart_handler.flush().await;

        if buffer[0] == 0xFF && buffer[1] == 0x86 {
            let co2_concentration  = ((buffer[2] as u16) << 8) | (buffer[3] as u16) ;

            Result::Ok(co2_concentration)
        } else {
            Result::Err("Invalid data from MHZ19B")
        }
    }
}
