#![no_std]
#![no_main]

use communicationmodule::sensors::{ sim808::Sim808, serial::Serial };

#[allow(unused)]
use esp_hal::{ clock::CpuClock, delay::Delay };

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_backtrace as _;

use esp_println::println;

#[allow(unused)]
use esp_wifi::{ EspWifiController, esp_now::{ EspNowReceiver, EspNowSender, PeerInfo } };

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
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let _ = spawner;


    let mut sim808 = Sim808::new(peripherals.UART1, peripherals.GPIO20, peripherals.GPIO21, 9600).unwrap();
    let mut serial = Serial::new(peripherals.UART0, peripherals.GPIO17, peripherals.GPIO16, 9600).unwrap();

    let mut sim808_buffer = [0u8; 64];

    let mut serial_buffer = [0u8; 64];


    println!("Enter AT Commands");

    loop {
        if let Ok(bytes_read) = serial.read_command(&mut serial_buffer).await {
            if bytes_read > 0 {
                sim808.send_command(&serial_buffer[..bytes_read]).await.unwrap();
                serial_buffer.fill(0);
            }
        }
        
        if let Ok(response_size) = sim808.read_response(&mut sim808_buffer).await {
            if response_size > 0 {
                serial.send_response(&sim808_buffer[..response_size]).await.unwrap();
                sim808_buffer.fill(0);
            }
        }
        
        Timer::after(Duration::from_secs(1)).await;
    }

}
