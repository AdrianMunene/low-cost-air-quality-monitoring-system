use crate::communicationprotocols::i2c::I2cHandler;
use bme280::i2c::BME280;

pub struct Bme280<'d> {
    bme280: I2cHandler<'d>
}

impl<'d> Bme280<'d> {
    pub fn new() -> Result<Self, Error> {

    }
}