#![no_std]
#![no_main]

use airqualitysensors::sensors::pms5003::Pms5003;
use airqualitysensors::sensors::mhz19b::Mhz19b;
//use airqualitysensors::sensors::bme280::Bme280;

use esp_backtrace as _;
use esp_hal::{ delay::Delay, prelude::* };

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let pms5003 = Pms5003::new(peripherals.UART1, peripherals.GPIO16, peripherals.GPIO17, 9600).unwrap();
    let mhz19b = Mhz19b::new(peripherals.UART0, peripherals.GPIO20, peripherals.GPIO21, 9600).unwrap();
    

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}
