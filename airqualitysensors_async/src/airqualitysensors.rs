use crate::sensors::{ mq7::Mq7, bme280::Bme280, mhz19b::Mhz19b, pms5003::Pms5003 };
use crate::communicationprotocols::pwm::PwmHandler;

use esp_hal::{
    mcpwm::PeripheralClockConfig,
    gpio::{ GpioPin, Output },
    delay::Delay,
    peripherals::{ADC1, I2C0, UART0, UART1, MCPWM0},
};
use embassy_time::{Timer, Duration};
use embassy_futures::join::join;

use libm::powf;
use fugit::RateExtU32;

pub struct AirQualitySensors {
    pub bme280: Bme280<'static>,
    pub mhz19b: Mhz19b<'static>,
    pub pms5003: Pms5003<'static>,
    pub mq7: Mq7<'static, GpioPin<3>>,
    pub activate_pin: Output<'static>,
    pub pwm_pin: PwmHandler<'static, MCPWM0>,
    pub last_co_reading: Option<u16>,
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
        gate_pin: GpioPin<10>,
        mcpwm: MCPWM0,
        pwm_pin: GpioPin<11>,
    ) -> Self {
        let peripheral_clock = PeripheralClockConfig::with_frequency(32.MHz()).unwrap();
        let mut delay = Delay::new();

        let activate_pin = Output::new(gate_pin, esp_hal::gpio::Level::Low);

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
            activate_pin,
            pwm_pin,
            last_co_reading: None,
        }
    }

    pub async fn read_uart_sensors(&mut self) -> ((u16, u16, u16), u16) {
        let (pm_data, co2_data) = join(self.pms5003.read_pm(), self.mhz19b.read_co2()).await;
        (pm_data.unwrap_or((999, 999, 999)), co2_data.unwrap_or(999))   
    }

    pub async fn read_bme280(&mut self) -> (f32, f32, f32) {
        let mut delay = Delay::new();
        let bme_data = self.bme280.measure(&mut delay).unwrap();
        (bme_data.temperature, bme_data.pressure, bme_data.humidity)
    }

    pub async fn read_mq7(&mut self) -> u16 {
        self.pwm_pin.set_duty_value(99).unwrap();

        let sample_count = 120; // 60s / 0.5s sampling interval
        let mut adc_sum: u32 = 0;

        for _ in 0..sample_count {
            let reading = self.mq7.read().unwrap_or(999);
            if reading != 999 {
                adc_sum += reading as u32;
            }
            Timer::after(Duration::from_millis(500)).await;
        }

        let avg_reading = if adc_sum == 0 {
            999
        } else {
            (adc_sum / sample_count) as u16
        };

        self.pwm_pin.set_duty_value(28).unwrap();
        Timer::after(Duration::from_secs(90)).await;

        let co = self.calculate_ppm(avg_reading);

        co

    }

    pub async fn read_all(&mut self) -> ((f32, f32, f32), (u16, u16, u16), u16, Option<u16>) {
        self.activate_pin.set_high();

        let ((pm1_0, pm2_5, pm10), co2) = self.read_uart_sensors().await;

        let (temperature, pressure, humidity) = self.read_bme280().await;

        (
            (temperature, pressure, humidity),
            (pm1_0, pm2_5, pm10),
            co2,
            self.last_co_reading,
        )
        
    }

    fn calculate_ppm(&self, reading: u16) -> u16 {
        const ADC_MAX: f32 = 4095.0;
        const V_REF: f32 = 3.3;
        const VOLTAGE_DIVIDER_RATIO: f32 = 3.3 / (2.0 + 3.3); // ~0.6226
        const INV_VOLTAGE_DIVIDER: f32 = 1.0 / VOLTAGE_DIVIDER_RATIO; // ~1.606

        const VC: f32 = 5.0;       // sensor supply voltage
        const RL: f32 = 10_000.0;  // load resistor ohms (check your board)
        const R0: f32 = 556.0;     // calibrated baseline resistance in clean air
        const A: f32 = 99.042;     // calibration constant A
        const B: f32 = 1.518;      // calibration constant B

        if reading == 999 {
            return 999;
        }

        // Convert ADC reading to voltage at ADC pin
        let v_adc = (reading as f32 / ADC_MAX) * V_REF;

        // Correct for voltage divider to get sensor output voltage
        let v_aout = v_adc * INV_VOLTAGE_DIVIDER;

        if v_aout <= 0.0 || v_aout >= VC {
            return 0;
        }

        // Calculate sensor resistance Rs
        let rs = RL * (VC - v_aout) / v_aout;

        // Calculate Rs/R0 ratio
        let ratio = rs / R0;

        // Apply standard MQ-7 calibration power-law formula
        let ppm = A * powf(ratio, -B);

        ppm as u16
    }
}
