#![no_std]
#![feature(type_alias_impl_trait)]

#[cfg(feature = "esp32")]
pub use esp32_hal as hal;
#[cfg(feature = "esp32c2")]
pub use esp32c2_hal as hal;
#[cfg(feature = "esp32c3")]
pub use esp32c3_hal as hal;
#[cfg(feature = "esp32c6")]
pub use esp32c6_hal as hal;
#[cfg(feature = "esp32h2")]
pub use esp32h2_hal as hal;
#[cfg(feature = "esp32s2")]
pub use esp32s2_hal as hal;
#[cfg(feature = "esp32s3")]
pub use esp32s3_hal as hal;

use embassy_time::{Duration, Ticker, Timer};
use esp_wifi::esp_now::{EspNow, EspNowError, PeerInfo};
use hal::embassy;
use keyboard_pipe::PIPE;
use log::{error, info};
use usb_zx::{
    uart_usb_key::{uart_code_to_usb_key, uart_composite_code_to_usb_key},
    usb_zx_key::usb_code_to_zxkey,
    zx_event::Event,
};

const ESP_NOW_PAYLOAD_INDEX: usize = 20;

#[embassy_executor::task]
pub async fn esp_now_receiver(esp_now: EspNow<'static>) {
    info!("ESP-NOW receiver task");
    let peer_info = PeerInfo {
        // Specify a unique peer address here (replace with actual address)
        peer_address: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
        lmk: None,
        channel: Some(1), // Specify the channel if known
        encrypt: false,   // Set to true if encryption is needed
    };

    // Check if the peer already exists
    if !esp_now.peer_exists(&peer_info.peer_address) {
        info!("Adding peer");
        match esp_now.add_peer(peer_info) {
            Ok(_) => info!("Peer added"),
            Err(e) => error!("Peer add error: {:?}", e),
        }
    } else {
        info!("Peer already exists, not adding");
    }

    loop {
        let received_data = esp_now.receive();
        match received_data {
            Some(data) => {
                let bytes = data.data;
                info!(
                    "Key code received over ESP-NOW: state = {:?}, modifier = {:?}, key = {:?}",
                    bytes[ESP_NOW_PAYLOAD_INDEX],
                    bytes[ESP_NOW_PAYLOAD_INDEX + 1],
                    bytes[ESP_NOW_PAYLOAD_INDEX + 2]
                );
                let bytes_written = PIPE
                    .write(&[
                        bytes[ESP_NOW_PAYLOAD_INDEX],
                        bytes[ESP_NOW_PAYLOAD_INDEX + 1],
                        bytes[ESP_NOW_PAYLOAD_INDEX + 2],
                    ])
                    .await;
                if bytes_written != 3 {
                    error!("Failed to write to pipe");
                    break;
                }
            }
            None => {
                //error!("ESP-NOW receive error");
            }
        }
        Timer::after(Duration::from_millis(5)).await;
    }
}
