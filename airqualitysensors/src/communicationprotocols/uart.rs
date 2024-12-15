use esp_hal::{
    gpio::interconnect::{ PeripheralInput, PeripheralOutput}, 
    peripheral::Peripheral, 
    uart::{AnyUart, Instance, Uart, Config, Error}, 
    Blocking,
};

use core::result::Result;

pub struct UartHandler<'d> {
   uart: Uart<'d, Blocking, AnyUart>,
}

impl<'d> UartHandler<'d> {
    pub fn new(
        uart: impl Peripheral<P = impl Instance> + 'd, 
        rx: impl Peripheral<P = impl PeripheralInput> + 'd, 
        tx: impl Peripheral<P = impl PeripheralOutput> + 'd, 
        baudrate: u32,) -> Result<Self, Error> {

            let config = Config::default().baudrate(baudrate);

            let uart = Uart::new_with_config(uart, config, rx, tx).unwrap();

            Result::Ok(Self { uart })
        }

    pub fn write(&mut self, data: &[u8],) -> Result<usize, Error> {
        self.uart.write_bytes(data)
    }

    pub fn read(&mut self, buffer: &mut [u8],) -> Result<(), Error> {
        self.uart.read_bytes(buffer)
    }
}


