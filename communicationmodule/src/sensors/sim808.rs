use crate::communicationprotocols::uart::UartHandler;

use esp_hal::{
    gpio::interconnect::{ PeripheralOutput, PeripheralInput },
    uart::{ Instance, Error },
    peripheral::Peripheral,
};

use core::result::Result;

pub struct Sim808<'d> {
    uart_handler: UartHandler<'d> 
}

impl<'d> Sim808<'d> {
    pub fn new(
    uart:impl Peripheral<P = impl Instance> + 'd,
    rx:impl Peripheral<P = impl PeripheralInput> + 'd,
    tx: impl Peripheral<P = impl PeripheralOutput> + 'd,
    baudrate: u32,) -> Result<Self, Error> {
        let uart_handler = UartHandler::new(uart, rx, tx, baudrate).unwrap();

        Result::Ok(Self{ uart_handler })
    }

    pub async fn send_command(&mut self, data: &[u8]) -> Result<usize, Error> {
        self.uart_handler.write(data).await
    }

    pub async fn read_response(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.uart_handler.read(buffer).await
    }
}
