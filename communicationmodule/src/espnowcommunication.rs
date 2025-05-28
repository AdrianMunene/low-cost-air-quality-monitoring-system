use esp_wifi::{EspWifiController, esp_now::{EspNow, EspNowReceiver, EspNowSender, PeerInfo}};
use esp_hal::peripherals::WIFI; 
use esp_println::println;

use embassy_time::{ Duration, Timer };

pub struct EspNowCommunicationManager<'d> {
    pub sender: EspNowSender<'d>,
    pub receiver: EspNowReceiver<'d>,
    pub peer_address: [u8; 6],
}

impl<'d> EspNowCommunicationManager<'d> {
    pub fn new(init: &'d EspWifiController, wifi: WIFI) -> Self {

        let esp_now = EspNow::new(init, wifi).unwrap();
        let (manager, sender, receiver) = esp_now.split();

        let peer_address = [0xf0, 0xf5, 0xbd, 0x0c, 0x0d, 0xc4]; 
        manager.add_peer(PeerInfo {
            peer_address,
            lmk: None,
            channel: None,
            encrypt: false,
        }).unwrap();

        EspNowCommunicationManager { sender, receiver, peer_address }
    }

    pub async fn send_data_request(sender: &mut EspNowSender<'d>, peer_address: &[u8; 6]) {
        let message = "REQUEST DATA";

        loop {
            match sender.send_async(peer_address, message.as_bytes()).await {
                Ok(_) => println!("ESP-NOW data request sent successfully"),
                Err(e) => println!("ESP-NOW data request send failed, {:?}", e),
            };

            Timer::after(Duration::from_secs(1)).await;
        }
    }

    pub async fn receive_sensor_data(&mut self) {
        loop {
            let data = self.receiver.receive_async().await;

            match core::str::from_utf8(data.data()) {
                Ok(received_data) => println!("Received Air Quality Data: {}", received_data),
                Err(_) => println!("Invalid UTF-8")
            };
        }
    }
}
