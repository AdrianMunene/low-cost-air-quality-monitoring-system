use crate::{ espnowcommunication::EspNowCommunicationManager }; 
use crate::sim808_functions::Sim808Functions;

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    rng::Rng,
    timer::{ systimer::SystemTimer, timg::TimerGroup }
};
use esp_wifi::{ esp_now::EspNowReceiver, EspWifiController };

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

static SENSOR_CHANNEL: Channel<CriticalSectionRawMutex, SensorData, 1> = Channel::new();


use esp_println::println;

use core::mem::MaybeUninit;

use heapless::Vec;

#[derive(Debug, Clone)]
pub struct SensorData {
    pub temperature: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub pm1_0: u16,
    pub pm2_5: u16,
    pub pm10: u16,
    pub co2: u16,
    pub co: u16,
    pub o3: u16,
}


static mut INIT: MaybeUninit<EspWifiController<'static>> = MaybeUninit::uninit();

#[embassy_executor::task]
async fn receiver_task(mut receiver: EspNowReceiver<'static>){
    loop {
        let data = receiver.receive_async().await;
        match core::str::from_utf8(data.data()) {
            Ok(text) => {
                println!("Received data: {}", text);

                let values: Vec<&str, 16> = text.split(',').collect();

                if values.len() == 8 {
                    if let (Ok(temp), Ok(press), Ok(hum), Ok(pm1), Ok(pm2), Ok(pm10), Ok(co2), Ok(co)) = (
                        values[0].parse::<f32>(),
                        values[1].parse::<f32>(),
                        values[2].parse::<f32>(),
                        values[3].parse::<u16>(),
                        values[4].parse::<u16>(),
                        values[5].parse::<u16>(),
                        values[6].parse::<u16>(),
                        values[7].parse::<u16>()
                    ) {
                        let sensor_data = SensorData {
                            temperature: temp,
                            pressure: press,
                            humidity: hum,
                            pm1_0: pm1,
                            pm2_5: pm2,
                            pm10: pm10,
                            co2: co2,
                            co: co,
                            o3: 0, 
                        };

                        SENSOR_CHANNEL.send(sensor_data).await;
                    } 
                }
            }
            Err(_) => println!("Received invalid UTF-8 data"),
        }
    }
}

#[embassy_executor::task]
pub async fn communication_main(spawner: Spawner) {
    
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


    let espnow_communication = EspNowCommunicationManager::new(
        init, 
        peripherals.WIFI
    );

    let receiver = espnow_communication.receiver;
    let mut sender = espnow_communication.sender;
    let peer_address = espnow_communication.peer_address;

    spawner.spawn(receiver_task(receiver)).unwrap();

    let mut sim808_functions = Sim808Functions::new(
        peripherals.UART0, 
        peripherals.GPIO17, 
        peripherals.GPIO16, 
        peripherals.UART1, 
        peripherals.GPIO20, 
        peripherals.GPIO21
    );
    

    loop {
        EspNowCommunicationManager::send_data_request(&mut sender, &peer_address).await;

        sim808_functions.config_sim808().await;

        let sensor_data = SENSOR_CHANNEL.receive().await;

        println!("Received sensor data: {:?}", sensor_data);

        sim808_functions.send_data(sensor_data.temperature, sensor_data.pressure, sensor_data.humidity, sensor_data.pm1_0, sensor_data.pm2_5, sensor_data.pm10, sensor_data.co2, sensor_data.co).await;

        Timer::after(Duration::from_secs(1)).await;
    }

}
