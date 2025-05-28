use embedded_hal::pwm::SetDutyCycle;
use esp_hal::{ 
    gpio::interconnect::PeripheralOutput,
    peripheral::Peripheral, 
    mcpwm::{ timer, operator::{ PwmPinConfig, PwmPin }, McPwm, PwmPeripheral, PeripheralClockConfig } 
};

use fugit::RateExtU32;

pub struct PwmHandler<'d, PWM> {
    pwm_pin: PwmPin<'d, PWM, 0, true>,
}

impl<'d, PWM> PwmHandler<'d, PWM>
where
    PWM: PwmPeripheral + 'd
{
    pub fn new(
        peripheral: impl Peripheral<P = PWM> + 'd, 
        peripheral_clock: PeripheralClockConfig, 
        pin: impl Peripheral<P = impl PeripheralOutput> + 'd) -> Self {

        let mut mcpwm = McPwm::new(peripheral, peripheral_clock);

        mcpwm.operator0.set_timer(&mcpwm.timer0);

        let pwm_pin = mcpwm.operator0.with_pin_a(pin, PwmPinConfig::UP_ACTIVE_HIGH);

        let timer_clock_cfg = peripheral_clock.timer_clock_with_frequency(
            99, 
            timer::PwmWorkingMode::Increase, 
            20.kHz()
        ).unwrap();

        mcpwm.timer0.start(timer_clock_cfg);

        Self { pwm_pin }
    }

    pub fn set_duty_value(&mut self, duty: u16) -> Result<(), &'static str> {
        self.pwm_pin.set_duty_cycle(duty).map_err(|_| "Failed to set duty value")
    }
}