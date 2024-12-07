use esp_hal::{
    i2c::master::{AnyI2c, Config, I2c, Instance, Error}, peripheral::Peripheral, Blocking
};
use esp_hal::gpio::interconnect::PeripheralOutput;
use core::result::Result;
use core::default::Default;

pub struct I2cHandler<'d> {
    i2c:I2c<'d, Blocking, AnyI2c>,
}

impl<'d> I2cHandler<'d>  {
    pub fn new(
        i2c: impl Peripheral<P = impl Instance> + 'd, 
        sda: impl Peripheral<P = impl PeripheralOutput> + 'd, 
        scl: impl Peripheral<P = impl PeripheralOutput> + 'd,) -> Result<Self, Error> {

       let i2c = I2c::new(i2c, Config::default()).with_sda(sda).with_scl(scl);

       Result::Ok(Self { i2c })
    }
}