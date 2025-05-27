use esp_hal::{
    gpio::AnalogPin,
    analog::adc::{Adc, AdcPin, AdcConfig, Attenuation, AdcChannel }, 
    peripherals::ADC1 
};

use core::result::Result;

pub struct AdcHandler<'d, PIN> {
    adc: Adc<'d, ADC1>,
    adc_pin: AdcPin<PIN, ADC1>
}

impl<'d, PIN> AdcHandler<'d, PIN> 
where 
    PIN: AdcChannel + AnalogPin
{
    pub fn new(adc: ADC1, pin: PIN) -> Self {
        let mut config = AdcConfig::new();

        let mut _adc_pin = config.enable_pin(pin, Attenuation::_11dB);

        let adc = Adc::new(adc, config);

        Self { adc, adc_pin: _adc_pin }
    }

    pub fn read(&mut self)-> Result<u16, ()> {
        nb::block!(self.adc.read_oneshot(&mut self.adc_pin))
    }
}
