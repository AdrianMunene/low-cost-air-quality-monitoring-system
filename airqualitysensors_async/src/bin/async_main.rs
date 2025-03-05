#![no_std]
#![no_main]

use airqualitysensors_async::sensors::{ bme280::Bme280, mhz19b::Mhz19b, pms5003::Pms5003 };

use esp_hal::{ clock::CpuClock, delay::Delay, gpio::Output };
use esp_wifi::{ EspWifiController, esp_now::{ EspNowReceiver, EspNowSender, PeerInfo } };
use esp_backtrace as _;
use esp_println::println;

use embassy_executor::Spawner;
use embassy_time::{ Duration, Timer };
use embassy_sync::{ blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal };
use embassy_futures::join::join;

use log::info;

extern crate alloc;
use alloc::format;

use core::mem::MaybeUninit;

static DATA_REQUEST_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
static mut INIT: MaybeUninit<EspWifiController<'static>> = MaybeUninit::uninit();


#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut delay = Delay::new();

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

    let peer_address = [0x40, 0x4c, 0xca, 0x4c, 0x44, 0xd8];

    manager.add_peer(PeerInfo { peer_address, lmk: None, channel: None, encrypt: false }).unwrap();

    let activate_pin = Output::new(peripherals.GPIO10, esp_hal::gpio::Level::High);
    let mhz19b = Mhz19b::new(peripherals.UART0, peripherals.GPIO17, peripherals.GPIO16, 9600).unwrap();
    let pms5003 = Pms5003::new(peripherals.UART1, peripherals.GPIO20, peripherals.GPIO21, 9600).unwrap(); 
    let mut bme280 = Bme280::new(peripherals.I2C0, peripherals.GPIO6, peripherals.GPIO7).unwrap();
    bme280.init(&mut delay);

    spawner.spawn(listen_for_signal(receiver)).unwrap();
    spawner.spawn(send_data(sender, peer_address, pms5003, mhz19b, bme280, activate_pin)).unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn listen_for_signal(mut receiver: EspNowReceiver<'static>) {
    loop {
        let data = receiver.receive_async().await;
        let received_data = core::str::from_utf8(data.data()).unwrap_or(""); 

        if received_data == "REQUEST DATA" {
            println!("Received signal from SIM808 node");
            DATA_REQUEST_SIGNAL.signal(());
        }
    }
}

#[embassy_executor::task]
async fn send_data(
    mut sender: EspNowSender<'static>, 
    peer_address: [u8; 6],
    mut pms5003: Pms5003<'static>,
    mut mhz19b: Mhz19b<'static>,
    mut bme280: Bme280<'static>, 
    mut activate_pin: Output<'static>,) {
    loop {

        DATA_REQUEST_SIGNAL.wait().await;

        activate_pin.set_high();

        let ((pm_data, co2_data), bme_data) = 
        join(
            join(read_pms5003(&mut pms5003), read_mhz19b(&mut mhz19b)), 
            read_bme280(&mut bme280)
        ).await;

        let(pm1_0, pm2_5, pm10) = pm_data;
        let co2 = co2_data;
        let (temperature, pressure, humidity) = bme_data;

        let airquality_data = format!(
            r#"{{ "co2": {}, "pm1_0": {}, "pm2_5": {}, "pm10": {}, "temperature": {:.3}, "pressure": {:.3}, "humidity": {:.3} }}"#, 
            co2, pm1_0, pm2_5, pm10, temperature, pressure, humidity
        );

        match sender.send_async(&peer_address, airquality_data.as_bytes()).await {
            Ok(_) => println!("ESP-NOW data sent successfully"),
            Err(e) => println!("ESP-NOW send failed, {:?}", e),
        };
    }
} 

async fn read_pms5003(pms5003: &mut Pms5003<'static>) -> (u16, u16, u16) {

    pms5003.read_pm().await.unwrap_or((999, 999, 999))

}

async fn read_mhz19b(mhz19b: &mut Mhz19b<'static>) -> u16 {
    
    mhz19b.read_co2().await.unwrap_or(999)

}

async fn read_bme280(bme280: &mut Bme280<'static>) -> (f32, f32, f32) {

        let mut delay = Delay::new();

        let measurements = bme280.measure(&mut delay).unwrap();
        (measurements.temperature, measurements.pressure, measurements.humidity)

}