use crate::{ espnowcommunication::EspNowCommunicationManager, sensors::{ serial::Serial, sim808::Sim808 } };
use crate::sim808_functions::Sim808Functions;

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    rng::Rng,
    timer::{ systimer::SystemTimer, timg::TimerGroup }
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_println::println;

#[embassy_executor::task]
pub async fn communication_main(_spawner: Spawner) {
    
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

    let mut espnow_communication = EspNowCommunicationManager::new(
        &init, 
        peripherals.WIFI
    );

    let _sim808_functions = Sim808Functions::new(
        peripherals.UART0, 
        peripherals.GPIO17, 
        peripherals.GPIO16, 
        peripherals.UART1, 
        peripherals.GPIO20, 
        peripherals.GPIO21
    );

    loop {
        espnow_communication.send_data_request().await;

        espnow_communication.receive_sensor_data().await;

        Timer::after(Duration::from_secs(1)).await;
    }

}

#[embassy_executor::task]
pub async fn interact_with_sim808(mut sim808: Sim808<'static>, mut serial: Serial<'static>) {

    let mut sim808_buffer = [0u8; 64];
    let mut serial_buffer = [0u8; 64];

    println!("Enter AT commands");

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