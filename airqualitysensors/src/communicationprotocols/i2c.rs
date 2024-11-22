use esp_hal::{
    gpio::Io, 
    i2c::master::I2c,
}
use esp_println::println;

pub struct I2cHandler<'d, DM: Mode, T = i2c::master::AnyI2c> {
    pub i2c: I2c<'d, DM, T>
}

impl <'d, DM, T> I2cHandler<'d, DM, T> {
    pub fn new(sda:, scl:,)
}