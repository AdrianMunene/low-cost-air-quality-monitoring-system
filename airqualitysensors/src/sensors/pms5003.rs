use crate::communicationprotocols::uart::{ UartHandler, SharedUart };

use core::result::Result;

pub struct Pms5003<'d> {
    uart_handler: UartHandler<'d>,
}

impl<'d> Pms5003<'d> {
    pub fn new(shared_uart: &'d SharedUart<'d>)  -> Self {
        let uart_handler = UartHandler::new(shared_uart);

        Self { uart_handler }
    }

    pub fn read_pm(&mut self) -> Result<(u16, u16, u16), &'static str> {
        let mut buffer = [0u8; 32];

        self.uart_handler.read(&mut buffer).map_err(|_| "Failed to read data from PMS5003")?;

        if buffer[0] == 0x42 && buffer[1] == 0x4D {
            let pm1_0 = ((buffer[10] as u16) << 8) | (buffer[11] as u16);
            let pm2_5 = ((buffer[12] as u16) << 8) | (buffer[13] as u16);
            let pm10 = ((buffer[14] as u16) << 8) | (buffer[15] as u16);

            Result::Ok((pm1_0, pm2_5, pm10))
        } else {
            Result::Err("Invalid header bytes from PMS5003 data")
        }

    }
}