use esp_hal::{
    gpio::interconnect::{ PeripheralInput, PeripheralOutput}, 
    peripheral::Peripheral, 
    uart::{ Instance, Uart, Config, Error}, 
    Async,
};

use core::result::Result;

pub struct UartHandler<'d> {
   uart: Uart<'d, Async>,
}

impl<'d> UartHandler<'d> {
    pub fn new(
        uart: impl Peripheral<P = impl Instance> + 'd, 
        rx: impl Peripheral<P = impl PeripheralInput> + 'd, 
        tx: impl Peripheral<P = impl PeripheralOutput> + 'd, 
        baudrate: u32,) -> Result<Self, Error> {

            let config = Config::default().with_baudrate(baudrate);

            let uart = Uart::new(uart, config,).unwrap().with_rx(rx).with_tx(tx);
            let uart = uart.into_async();

            Result::Ok(Self { uart })
        }

    pub async fn write(&mut self, data: &[u8],) -> Result<usize, Error> {
        self.uart.write_async(data).await
    }

    pub async fn read(&mut self, buffer: &mut [u8],) -> Result<usize, Error> {
        self.uart.read_async(buffer).await
    }

    pub async fn flush(&mut self) {
        self.uart.flush_async().await.unwrap();
    }
}

