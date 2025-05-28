use crate::espnowcommunication::EspNowCommunicationManager;
use crate::airqualitysensors::AirQualitySensors;

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    rng::Rng,
    timer::{ systimer::SystemTimer, timg::TimerGroup },
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use alloc::format;


#[embassy_executor::task]
pub async fn airquality_main(_spawner: Spawner) {
    // Initialize HAL
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let systimer = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(systimer.alarm0);

    let _delay = Delay::new();

    let timer = TimerGroup::new(peripherals.TIMG0);

    let init = esp_wifi::init(
        timer.timer0, 
        Rng::new(peripherals.RNG), 
        peripherals.RADIO_CLK
    ).unwrap();

    // Initialize ESP-NOW communication
    let mut espnow_communication = EspNowCommunicationManager::new(
        &init, 
        peripherals.WIFI
    );

    // Initialize sensors
    let mut sensors = AirQualitySensors::new(
        peripherals.ADC1,
        peripherals.GPIO3,
        peripherals.I2C0,
        peripherals.GPIO6,
        peripherals.GPIO7,
        peripherals.UART0,
        peripherals.GPIO17,
        peripherals.GPIO16,
        peripherals.UART1,
        peripherals.GPIO20,
        peripherals.GPIO21,
        peripherals.GPIO10,
        peripherals.MCPWM0,
        peripherals.GPIO11,
    );

    loop {
        espnow_communication.wait_for_request().await;
        let (environment_variables, pm, co2, co) = sensors.read_all().await;

        let (pm1_0, pm2_5, pm10) = pm;
        let (temperature, pressure, humidity) = environment_variables;

        let payload = format!(
            r#"{{ "temperature": {:.2}, "pressure": {:.2}, "humidity": {:.2}, "pm1_0": {}, "pm2_5": {}, "pm10": {}, "co2": {}, "co": {} }}"#,
            temperature, pressure, humidity, pm1_0, pm2_5, pm10, co2, co
        );

        espnow_communication.send_response(&payload).await;
        Timer::after(Duration::from_secs(2)).await;
    }
}
