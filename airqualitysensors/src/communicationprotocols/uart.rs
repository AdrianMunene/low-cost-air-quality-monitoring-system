use esp_hal::{
    peripheral::Peripheral, uart::{AnyUart, Config, Error, Uart}, Blocking, 
};
use esp_hal::gpio::interconnect::{PeripheralInput, PeripheralOutput};
use core::result::Result;
use core::default::Default;
pub struct UartHandler<'d> {
    uart: Uart<'d, Blocking, AnyUart>,
}

impl<'d> UartHandler<'d> {
    pub fn new(
        rx: impl Peripheral<P = impl PeripheralInput> + 'd, 
        tx: impl Peripheral<P = impl PeripheralOutput> + 'd, 
        baudrate: u32,) -> Result<Self, Error> {

            let peripherals = esp_hal::init(esp_hal::Config::default());

            let config = Config::default().baudrate(baudrate);

            let uart = Uart::new_with_config(peripherals.UART1, config, rx, tx)?;

            Result::Ok(Self { uart })

    }

    pub fn write(&mut self, data: &[u8],) -> Result<usize, Error> {
        self.uart.write_bytes(data)
    }

    pub fn read(&mut self, buffer: &mut [u8],) -> Result<(), Error> {
        self.uart.read_bytes(buffer)
    }
}

