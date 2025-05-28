#![no_std]
#![no_main]

use embassy_executor::Spawner;
use communicationmodule::communication::communication_main;

use esp_backtrace as _;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let _ = spawner.spawn(communication_main(spawner));
}