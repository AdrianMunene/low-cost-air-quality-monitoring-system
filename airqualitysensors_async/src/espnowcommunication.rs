use esp_wifi::{EspWifiController, esp_now::{EspNow, EspNowReceiver, EspNowSender, PeerInfo}};
use esp_hal::peripherals::WIFI; 
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use esp_println::println;

static DATA_REQUEST_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub struct EspNowCommunicationManager<'d> {
    pub sender: EspNowSender<'d>,
    pub receiver: EspNowReceiver<'d>,
    pub peer_address: [u8; 6],
}

impl<'d> EspNowCommunicationManager<'d> {
    pub fn new(init: &'d EspWifiController, wifi: WIFI) -> Self {

        let esp_now = EspNow::new(init, wifi).unwrap();
        let (manager, sender, receiver) = esp_now.split();

        let peer_address = [0xf0, 0xf5, 0xbd, 0x0b, 0xfe, 0x88];
        manager.add_peer(PeerInfo {
            peer_address,
            lmk: None,
            channel: None,
            encrypt: false,
        }).unwrap();

        EspNowCommunicationManager { sender, receiver, peer_address }
    }

    pub async fn wait_for_request(&mut self) {
        loop {
            let data = self.receiver.receive_async().await;
            if core::str::from_utf8(data.data()).unwrap_or("") == "REQUEST DATA" {
                DATA_REQUEST_SIGNAL.signal(());
            }
        }
    }

    pub async fn wait_for_signal(&self) {
        DATA_REQUEST_SIGNAL.wait().await;
    }

    pub async fn send_response(&mut self, payload: &str) {
        match self.sender.send_async(&self.peer_address, payload.as_bytes()).await {
            Ok(_) => println!("ESP-NOW data sent: {}", payload),
            Err(e) => println!("ESP-NOW send failed: {:?}", e),
        }
    }
}
