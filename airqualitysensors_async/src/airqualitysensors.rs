use crate::sensors::{ mq7::Mq7, bme280::Bme280, mhz19b::Mhz19b, pms5003::Pms5003 };
use crate::communicationprotocols::pwm::PwmHandler;
use esp_hal::mcpwm::PeripheralClockConfig;
use esp_hal::{
    gpio::GpioPin,
    delay::Delay,
    peripherals::{ADC1, I2C0, UART0, UART1, MCPWM0},
};
use embassy_time::{Timer, Duration};
use libm::powf;
use fugit::RateExtU32;

pub struct AirQualitySensors {
    pub bme280: Bme280<'static>,
    pub mhz19b: Mhz19b<'static>,
    pub pms5003: Pms5003<'static>,
    pub mq7: Mq7<'static, GpioPin<3>>,
    pub pwm_pin: PwmHandler<'static, MCPWM0>
}

impl AirQualitySensors {
    pub fn new(
    adc: ADC1,
    adc_pin: GpioPin<3>,
    i2c: I2C0,
    sda: GpioPin<6>,
    scl: GpioPin<7>,
    uart0: UART0,
    rx0: GpioPin<17>,
    tx0: GpioPin<16>,
    uart1: UART1,
    rx1: GpioPin<20>,
    tx1: GpioPin<21>,
    mcpwm: MCPWM0,
    pwm_pin: GpioPin<11>,) -> Self {
        let peripheral_clock = PeripheralClockConfig::with_frequency(32.MHz()).unwrap();
        let mut delay = Delay::new();

        let mq7 = Mq7::new(adc, adc_pin);
        let pwm_pin = PwmHandler::new(mcpwm, peripheral_clock, pwm_pin);

        let mut bme280 = Bme280::new(i2c, sda, scl).unwrap();
        bme280.init(&mut delay);

        let mhz19b = Mhz19b::new(uart0, rx0, tx0, 9600).unwrap();
        let pms5003 = Pms5003::new(uart1, rx1, tx1, 9600).unwrap();


        AirQualitySensors {
            bme280,
            mhz19b,
            pms5003,
            mq7,
            pwm_pin
        }
    }

    pub async fn read_all(&mut self) -> ((f32, f32, f32), (u16, u16, u16), u16,  u16) {
        self.pwm_pin.set_duty_value(99).unwrap();
        Timer::after(Duration::from_secs(60)).await;

        self.pwm_pin.set_duty_value(28).unwrap();
        Timer::after(Duration::from_secs(90)).await;


        let mq7_reading = self.mq7.read().unwrap_or(999);
        let co = self.calculate_ppm(mq7_reading);

        let pm_data = self.pms5003.read_pm().await.unwrap_or((999, 999, 999));
        let co2 = self.mhz19b.read_co2().await.unwrap_or(999);

        let mut delay = Delay::new();
        let measurements = self.bme280.measure(&mut delay).unwrap();

        ((measurements.temperature, measurements.pressure, measurements.humidity), pm_data, co2,  co)
    }

    fn calculate_ppm(&self, reading: u16) -> u16 {
        const R0: u16 = 556;
        const A: f32 = 99.042;
        const B: f32 = 1.518;

        if reading == 999 {
            return reading
        }

        let ratio = reading as f32 / R0 as f32;
        let ppm = powf(ratio * A, 1.0 / B);
        ppm as u16
    }
}
