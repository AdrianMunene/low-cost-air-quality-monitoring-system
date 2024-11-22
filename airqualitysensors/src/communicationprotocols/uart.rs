use esp_hal::{
    uart::{Uart, config::Config},
    gpio::Io,
}
use esp_println::println;

pub struct UartHandler<'d, M, T = uart::AnyUart> {
    uart: Uart<'d, M, T>,
}

impl<'d, M> UartHandler<'d, M> {

    pub fn new(
        rx: impl Peripheral<P = PeripheralInput> + 'd, 
        tx: impl Peripheral<P = PeripheralOutput> + 'd, 
        baudrate: Hertz) -> Self {
            let uart = Uart::new_with_config(peripherals.UART1, Config::default().baudrate(baudrate), rx, tx).unwrap();

            UartHandler {uart}
        }

    pub fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        self.uart.write_bytes(data)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        self.uart.read_bytes(buffer)
    }
}