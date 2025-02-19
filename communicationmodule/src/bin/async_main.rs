#![no_std]
#![no_main]

#[allow(unused)]
use communicationmodule::sensors::{ sim808::Sim808, serial::Serial };

use esp_hal::clock::CpuClock;
use esp_wifi::{ EspWifiController, esp_now::{ EspNowReceiver, EspNowSender, PeerInfo } };
use esp_backtrace as _;
use esp_println::println;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use log::info;

extern crate alloc;
use core::mem::MaybeUninit;

static mut INIT: MaybeUninit<EspWifiController<'static>> = MaybeUninit::uninit();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timer1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);

    let init = unsafe{ 
        INIT.write(esp_wifi::init(timer1.timer0, esp_hal::rng::Rng::new(peripherals.RNG), peripherals.RADIO_CLK,).unwrap()); 
        
        INIT.assume_init_mut()
    };

    
    let esp_now = esp_wifi::esp_now::EspNow::new(init, peripherals.WIFI).unwrap();
    let (manager, sender, receiver) = esp_now.split(); 

    let peer_address = [0x40, 0x4c, 0xca, 0x4c, 0x6f, 0x18];
    manager.add_peer(PeerInfo { peer_address, lmk: None, channel: None, encrypt: false }).unwrap();

    let mut sim808 = Sim808::new(peripherals.UART1, peripherals.GPIO20, peripherals.GPIO21, 9600).unwrap();
    let mut _serial = Serial::new(peripherals.UART0, peripherals.GPIO17, peripherals.GPIO16, 9600).unwrap();

    config_sim808(&mut sim808).await;

    spawner.spawn(send_data_request(sender, peer_address)).unwrap();
    spawner.spawn(receive_sensor_data(receiver)).unwrap();

    loop {
       Timer::after(Duration::from_secs(1)).await;
    }

}

#[embassy_executor::task]
async fn send_data_request(
    mut sender: EspNowSender<'static>, 
    peer_address: [u8;6]) {
    
    let message = "REQUEST DATA";

    loop {
        match sender.send_async(&peer_address, message.as_bytes()).await {
            Ok(_) => {},
            //println!("ESP-NOW data request sent successfully"),
            Err(e) => println!("ESP-NOW data request send failed, {:?}", e),
        };
    }
}

#[embassy_executor::task]
async fn receive_sensor_data(mut receiver: EspNowReceiver<'static>) {
    loop {
        let data = receiver.receive_async().await;

        let received_data = core::str::from_utf8(data.data()).unwrap_or("Invalid UTF-8");

        println!("Received Air Quality Data: {}", received_data);
    }
}

async fn config_sim808(sim808: &mut Sim808<'_>) {
    
    let mut buffer = [0u8; 64];

    sim808.send_command("AT+SAPBR=1,1".as_bytes()).await.unwrap();

    if let Ok(bytes_read) = sim808.read_response(&mut buffer).await {
        if bytes_read > 0 {
            if buffer[..bytes_read] == *"ERROR".as_bytes() {
                println!("Error sending AT+SAPBR=1,1 Command");
            } else{
                println!("SIM808 responded with: {:?}", buffer);
            }
            buffer.fill(0);
        }
    }

    sim808.send_command("AT+CGNSPWR=1".as_bytes()).await.unwrap();

    if let Ok(bytes_read) = sim808.read_response(&mut buffer).await {
        if bytes_read > 0 {
            if buffer[..bytes_read] == *"ERROR".as_bytes() {
                println!("Error sending AT+CGNSPWR=1 Command");
            } else {
                println!("SIM808 responded with {:?}", buffer);
            }
            buffer.fill(0);
        }
    }

}

#[embassy_executor::task]
async fn interact_with_sim808(mut sim808: Sim808<'static>, mut serial: Serial<'static>) {

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