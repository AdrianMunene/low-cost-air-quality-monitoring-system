use crate::{espnowcommunication::EspNowCommunicationManager, sensors};
use crate::airqualitysensors::AirQualitySensors;

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    rng::Rng,
    timer::{ systimer::SystemTimer, timg::TimerGroup },
};

use esp_wifi::{ EspWifiController, esp_now::EspNowReceiver };

use embassy_executor::Spawner;

use alloc::format;

use core::mem::MaybeUninit;

static mut INIT: MaybeUninit<EspWifiController<'static>> = MaybeUninit::uninit();
static mut SENSORS: MaybeUninit<AirQualitySensors> = MaybeUninit::uninit();

#[embassy_executor::task]
async fn listener_task(receiver: EspNowReceiver<'static>) {
    EspNowCommunicationManager::wait_for_request(receiver).await;
}

#[embassy_executor::task]
async fn read_mq7(sensors_ptr: *mut AirQualitySensors){
    loop {
        let sensors = unsafe { &mut *sensors_ptr };

        let co = sensors.read_mq7().await;
        sensors.last_co_reading = Some(co);
    }
}


#[embassy_executor::task]
pub async fn airquality_main(spawner: Spawner) {
    // Initialize HAL
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let systimer = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(systimer.alarm0);

    let _delay = Delay::new();

    let timer = TimerGroup::new(peripherals.TIMG0);

    let init = unsafe{ 
        INIT.write(esp_wifi::init(timer.timer0, Rng::new(peripherals.RNG), peripherals.RADIO_CLK,).unwrap()); 
        
        INIT.assume_init_mut()
    };

    // Initialize ESP-NOW communication
    let espnow_communication = EspNowCommunicationManager::new(
        init, 
        peripherals.WIFI
    );

    let receiver = espnow_communication.receiver;
    let mut sender = espnow_communication.sender;
    let peer_address = espnow_communication.peer_address;

    spawner.spawn(listener_task(receiver)).unwrap();

    // Initialize sensors
    let sensors = unsafe { 
        SENSORS.write(AirQualitySensors::new(
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
        ));

        SENSORS.assume_init_mut()
    };

    let sensors_ptr = sensors as *mut AirQualitySensors;
    spawner.spawn(read_mq7(sensors_ptr)).unwrap();

    loop {
        EspNowCommunicationManager::wait_for_signal().await;
        let (environment_variables, pm, co2, co) = sensors.read_all().await;

        let (pm1_0, pm2_5, pm10) = pm;
        let (temperature, pressure, humidity) = environment_variables;
        let co = co.unwrap_or(999); 

        let payload = format!(
            r#"{{ "temperature": {:.2}, "pressure": {:.2}, "humidity": {:.2}, "pm1_0": {}, "pm2_5": {}, "pm10": {}, "co2": {}, "co": {} }}"#,
            temperature, pressure, humidity, pm1_0, pm2_5, pm10, co2, co
        );

        EspNowCommunicationManager::send_response(&mut sender, &peer_address, &payload).await;
    }
}
