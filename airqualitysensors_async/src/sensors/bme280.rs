use crate::communicationprotocols::i2c::I2cHandler;

use bme280::i2c::BME280;

use esp_hal::{
    gpio::interconnect::PeripheralOutput,
    i2c::master::{I2c, Error, Instance}, 
    peripheral::Peripheral, 
    delay::Delay,
    Async, 
};

use core::result::Result;

pub struct Bme280<'d> {
    bme280: BME280<I2c<'d, Async>>
}

impl<'d> Bme280<'d> {
    pub fn new(i2c: impl Peripheral<P = impl Instance> + 'd, 
    sda: impl Peripheral<P = impl PeripheralOutput> + 'd, 
    scl: impl Peripheral<P = impl PeripheralOutput> + 'd,) -> Result<Self, Error> {

        let i2c = I2cHandler::new(i2c, sda, scl).unwrap();

        let bme280 = BME280::new_primary(i2c.get_inner_i2c()); 

        Result::Ok(Self { bme280 })

    }

    pub fn init(&mut self, delay: &mut Delay) {
        self.bme280.init(delay).unwrap();
    }

    pub fn measure(&mut self, delay: &mut Delay) -> Result<bme280::Measurements<Error>, Error> {
        Result::Ok(self.bme280.measure(delay).unwrap())
    }
}