#![no_std]
#![no_main]

use airqualitysensors::sensors::pms5003::Pms5003;
use airqualitysensors::sensors::mhz19b::Mhz19b;
use airqualitysensors::sensors::bme280::Bme280;

use airqualitysensors::communicationprotocols::uart::SharedUart;

use esp_backtrace as _;
use esp_hal::{ uart::{ Uart, Config }, delay::Delay, prelude::* };
use esp_println::println;

use core::cell::RefCell;

use critical_section::Mutex;


static UART1: SharedUart = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();
    let config = Config::default().baudrate(9600);

    let uart1 = Uart::new_with_config(peripherals.UART1, config , peripherals.GPIO20, peripherals.GPIO21).unwrap();

    critical_section::with(|cs| *UART1.borrow_ref_mut(cs) = Some(uart1));

    let mut pms5003 = Pms5003::new(&UART1);
    let mut mhz19b = Mhz19b::new(&UART1);

    let mut bme280 = Bme280::new(peripherals.I2C0, peripherals.GPIO6, peripherals.GPIO7).unwrap();
    bme280.init(&mut delay);

    loop {

        //PMS5003
        /*if let Ok((pm1_0, pm2_5, pm10)) = pms5003.read_pm() {
            println!("PMS5003: PM1.0: {}μg/m3, PM2.5: {}μg/m3, PM10: {}ug/m3", pm1_0, pm2_5, pm10);
        }*/

        // MHZ19B
        if let Ok(co2) = mhz19b.read_co2() {
            println!("MHZ19B: CO2: {} ppm", co2);
        }

        //BME280
        /*if let Ok(measurements) = bme280.measure(&mut delay) {
            println!("BME280: Temperature: {}°C, Humidity: {}%, Pressure{}pa", measurements.temperature, measurements.humidity, measurements.pressure);
        }*/

        delay.delay(1000.millis());
    }
}
