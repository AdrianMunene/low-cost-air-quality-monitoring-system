use esp_hal::{
    gpio::interconnect::PeripheralOutput,
    i2c::master::{Config, I2c, Instance, Error}, 
    peripheral::Peripheral, 
    Async,
};

use core::{ 
    result::Result, 
    default::Default, 
};

pub struct I2cHandler<'d> {
    i2c:I2c<'d, Async>,
}

impl<'d> I2cHandler<'d>  {
    pub fn new(
        i2c: impl Peripheral<P = impl Instance> + 'd, 
        sda: impl Peripheral<P = impl PeripheralOutput> + 'd, 
        scl: impl Peripheral<P = impl PeripheralOutput> + 'd,) -> Result<Self, Error> {

       let i2c = I2c::new(i2c, Config::default()).unwrap().with_sda(sda).with_scl(scl);
       let i2c = i2c.into_async();

       Result::Ok(Self { i2c })
    }

    pub fn get_inner_i2c(self) -> I2c<'d, Async> {
        self.i2c
    } 
}