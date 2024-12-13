use esp_hal::{ 
    uart::{ Config, lp_uart::LpUart, Error }, 
    peripherals::LP_UART,
}; 

use core::result::Result;

pub struct LpUartHandler {
    lpuart: LpUart
}

impl  LpUartHandler {
    pub fn new(uart: LP_UART, baudrate: u32) -> Result<Self, Error> {

        let config = Config::default().baudrate(baudrate);

        let lpuart = LpUart::new_with_config(uart, config);

        Result::Ok(Self { lpuart })

    }
}