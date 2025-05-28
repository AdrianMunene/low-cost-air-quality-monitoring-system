#![no_std]
#![no_main]

use embassy_executor::Spawner;
use airqualitysensors_async::airquality::airquality_main;

use esp_backtrace as _;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let _ = spawner.spawn(airquality_main(spawner));
}