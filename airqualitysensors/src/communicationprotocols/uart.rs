use esp_hal::{
    uart::{AnyUart, Uart}, Blocking 
};
use core::result::Result;
use core::cell::RefCell;

use critical_section::Mutex;

pub type SharedUart<'d> = Mutex<RefCell<Option<Uart<'d, Blocking, AnyUart>>>>;

pub struct UartHandler<'d> {
    shared_uart: &'d SharedUart<'d>,
}

impl<'d> UartHandler<'d> {
    pub fn new(shared_uart: &'d SharedUart<'d>) -> Self {
        Self { shared_uart }
    }

    pub fn write(&self, data: &[u8],) -> Result<usize, &'static str> {
        critical_section::with(|cs| {
            let mut uart = self.shared_uart.borrow_ref_mut(cs);
            if let Some(uart) = uart.as_mut() {
                uart.write_bytes(data).map_err(|_| "Write failed")
            } else {
                Err("Uart not initialised")
            }
        })
    }

    pub fn read(&self, buffer: &mut [u8],) -> Result<(), &'static str> {
        critical_section::with(|cs| {
            let mut uart = self.shared_uart.borrow_ref_mut(cs);
            if let Some(uart) = uart.as_mut() {
                uart.read_bytes(buffer).map_err(|_| "Read failed")
            } else {
                Err("Uart not initialised")
            }
        })
    }
}


