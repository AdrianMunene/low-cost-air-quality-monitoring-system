#![no_std]
#![no_main]

use airqualitysensors_async::sensors::{
    bme280::Bme280, 
    mhz19b::Mhz19b, 
    pms5003::Pms5003,
};

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_backtrace as _;

use log::info;

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timer1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(timer1.timer0, esp_hal::rng::Rng::new(peripherals.RNG), peripherals.RADIO_CLK,).unwrap();

    let mhz19b = Mhz19b::new(peripherals.UART0, peripherals.GPIO17, peripherals.GPIO16, 9600).unwrap();
    let pms5003 = Pms5003::new(peripherals.UART1, peripherals.GPIO20, peripherals.GPIO21, 9600).unwrap(); 
    let bme280 = Bme280::new(peripherals.I2C0, peripherals.GPIO6, peripherals.GPIO7).unwrap();

    spawner.spawn(read_pms5003(pms5003)).unwrap();
    spawner.spawn(read_mhz19b(mhz19b)).unwrap();
    spawner.spawn(read_bme280(bme280)).unwrap();


    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn read_pms5003(mut pms5003: Pms5003<'static>) {
    loop {
        info!("Reading PMS5003...");

        if let Ok((pm1_0, pm2_5, pm10)) = pms5003.read_pm().await {
            info!("PMS5003 - PM1.0: {}μg/m3, PM2.5: {}μg/m3, PM10: {}μg/m3", pm1_0, pm2_5, pm10);
        } else {
            info!("Failed to read PMS5003");
        }
    }
}

#[embassy_executor::task]
async fn read_mhz19b(mut mhz19b: Mhz19b<'static>) {
    loop {
        info!("Reading MH-Z19B...");

        if let Ok(co2) = mhz19b.read_co2().await {
            info!("MH-Z19B: CO2: {}, ppm", co2);
        } else {
            info!("Failed to read MH-Z19B");
        }
    }
}

#[embassy_executor::task]
async fn read_bme280(mut bme280: Bme280<'static>) {
    loop {
        info!("Reading BME280");
        let mut delay = Delay::new();

        if let Ok(measurements) = bme280.measure(&mut delay) {
            info!("BME280: Temperature: {}°C, Humidity: {}%, Pressure{}pa", measurements.temperature, measurements.humidity, measurements.pressure);
        } else {
            info!("Failed to read BME280");
        }
    }
}