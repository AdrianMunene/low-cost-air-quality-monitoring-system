use esp_wifi::{EspWifiController, esp_now::{EspNow, EspNowReceiver, EspNowSender, PeerInfo}};
use esp_hal::peripherals::WIFI; 

use esp_println::println;

use embassy_sync::{channel::Channel, blocking_mutex::raw::CriticalSectionRawMutex};
//use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

static REQUEST_CHANNEL: Channel<CriticalSectionRawMutex, (), 4> = Channel::new();

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

    pub fn take_receiver(self) -> EspNowReceiver<'d> {
        self.receiver
    }    

    pub fn take_sender(self) -> EspNowSender<'d> {
        self.sender
    }

    pub async fn wait_for_request(mut receiver: EspNowReceiver<'d>) {
        loop {
            let data = receiver.receive_async().await;
            if core::str::from_utf8(data.data()).unwrap_or("") == "REQUEST DATA" {
                let _ = REQUEST_CHANNEL.send(()).await;
            }
        }
    }

    pub async fn wait_for_signal() {
        REQUEST_CHANNEL.receive().await;
    }

    pub async fn send_response(sender: &mut EspNowSender<'d>, peer_address: &[u8; 6], payload: &str) {
        match sender.send_async(peer_address, payload.as_bytes()).await {
            Ok(_) => println!("ESP-NOW data sent: {}", payload),
            Err(e) => println!("ESP-NOW send failed: {:?}", e),
        }
    }
}
