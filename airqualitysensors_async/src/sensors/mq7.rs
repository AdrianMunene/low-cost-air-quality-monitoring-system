use crate::communicationprotocols::adc::AdcHandler;

use esp_hal::{
    gpio::AnalogPin,
    analog::adc::{ AdcChannel }, 
    peripherals::ADC1 
};

pub struct Mq7<'d, PIN>{
    adc_handler: AdcHandler<'d, PIN>,
}

impl<'d, PIN> Mq7<'d, PIN>
where
    PIN: AdcChannel + AnalogPin
{
    pub fn new(adc: ADC1, pin: PIN ) -> Self {
        let adc_handler = AdcHandler::new(adc, pin);

        Self { adc_handler }
    }

    pub fn read(&mut self)-> Result<u16, &'static str> {
        match self.adc_handler.read() {
            Ok(reading) => Result::Ok(reading),
            Err(()) => Result::Err("Failed to read MQ7")
        }
    }
}