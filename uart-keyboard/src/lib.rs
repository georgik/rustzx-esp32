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
use hal::{embassy, peripherals::UART0, Uart};
use keyboard_pipe::PIPE;
use log::{debug, error, info};
use usb_zx::{
    uart_usb_key::{uart_code_to_usb_key, uart_composite_code_to_usb_key},
    usb_zx_key::usb_code_to_zxkey,
    zx_event::Event,
};

async fn usb_write_key(key_state: u8, modifier: u8, key_code: u8) {
    let mut bytes = [0u8; 3];
    bytes[0] = key_state;
    bytes[1] = modifier;
    bytes[2] = key_code;
    let bytes_written = PIPE.write(&bytes).await;
    if bytes_written != 3 {
        error!("Failed to write to pipe");
    }
}

async fn usb_press_key(modifier: u8, key_code: u8) {
    usb_write_key(0, modifier, key_code).await;
    usb_write_key(1, modifier, key_code).await;
}

/// Read from UART and send to emulator
#[embassy_executor::task]
pub async fn uart_receiver(uart0: Uart<'static, UART0>) {
    info!("UART receiver task");

    let (_, mut rx) = uart0.split();
    const MAX_BUFFER_SIZE: usize = 16;
    let mut rbuf: [u8; MAX_BUFFER_SIZE] = [0u8; MAX_BUFFER_SIZE];

    loop {
        let result = embedded_io_async::Read::read(&mut rx, &mut rbuf).await;
        match result {
            Ok(bytes_read) => {
                info!("UART read: {} bytes", bytes_read);
                if bytes_read == 1 {
                    info!("UART read: {:x}", rbuf[0]);
                    match uart_code_to_usb_key(rbuf[0]) {
                        Some((modifier, key_code)) => {
                            usb_press_key(modifier, key_code).await;
                        }
                        None => {
                            error!("Unknown key code");
                        }
                    }
                } else if bytes_read == 3 {
                    info!("UART read: {:x} {:x} {:x}", rbuf[0], rbuf[1], rbuf[2]);
                    match uart_composite_code_to_usb_key(rbuf[0], rbuf[1], rbuf[2]) {
                        Some((modifier, key_code)) => {
                            usb_press_key(modifier, key_code).await;
                        }
                        None => {
                            error!("Unknown key code");
                        }
                    }
                }
            }
            Err(e) => {
                error!("UART read error: {:?}", e);
            }
        }

        Timer::after(Duration::from_millis(5)).await;
    }
}
