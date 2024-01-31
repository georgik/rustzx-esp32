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

pub mod host;
pub mod io;
pub mod stopwatch;

use rustzx_core::{host::Host, zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings};

use log::{debug, error, info};

use keyboard_pipe::PIPE;

use embassy_time::{Duration, Timer};

use esp_bsp::{define_display_type, BoardType};

use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    prelude::{Point, RgbColor},
    text::Text,
    Drawable,
};

use esp_display_interface_spi_dma::display_interface_spi_dma;
use hal::gpio::{GpioPin, Output};
use hal::spi::FullDuplexMode;

use usb_zx::{usb_zx_key::usb_code_to_zxkey, zx_event::Event};

use crate::io::FileAsset;

#[cfg(feature = "esp32_c6_devkit_c1")]
type AppDisplay = define_display_type!(BoardType::ESP32C6DevKitC1);
#[cfg(feature = "m5stack_cores3")]
type AppDisplay = define_display_type!(BoardType::M5StackCoreS3);
#[cfg(feature = "esp32_s3_box")]
type AppDisplay = define_display_type!(BoardType::ESP32S3Box);

const SCREEN_OFFSET_X: u16 = (320 - 256) / 2;
const SCREEN_OFFSET_Y: u16 = (240 - 192) / 2;

fn handle_key_event<H: Host>(
    key_state: u8,
    modifier: u8,
    key_code: u8,
    emulator: &mut Emulator<H>,
) {
    let is_pressed = key_state == 0;
    if let Some(mapped_key) = usb_code_to_zxkey(is_pressed, modifier, key_code) {
        match mapped_key {
            Event::ZXKey(k, p) => {
                debug!("-> ZXKey");
                emulator.send_key(k, p);
            }
            Event::NoEvent => {
                error!("Key not implemented");
            }
            Event::ZXKeyWithModifier(k, k2, p) => {
                debug!("-> ZXKeyWithModifier");
                emulator.send_key(k, p);
                emulator.send_key(k2, p);
            }
        }
    } else {
        info!("Mapped key: NoEvent");
    }
}

#[embassy_executor::task]
pub async fn app_loop(mut display: AppDisplay)
//-> Result<(), core::fmt::Error>
{
    // let _ = lcd_backlight.set_high();

    Timer::after(Duration::from_millis(500)).await;

    info!("Initializing...");
    Text::new(
        "Initializing...",
        Point::new(80, 110),
        MonoTextStyle::new(&FONT_8X13, RgbColor::WHITE),
    )
    .draw(&mut display)
    .unwrap();

    info!("Initialized");

    // display
    //     .clear(color_conv(ZXColor::Blue, ZXBrightness::Normal))
    //     .map_err(|err| error!("{:?}", err))
    //     .ok();

    info!("Creating emulator");

    let settings = RustzxSettings {
        machine: ZXMachine::Sinclair128K,
        // machine: ZXMachine::Sinclair48K,
        emulation_mode: EmulationMode::FrameCount(1),
        tape_fastload_enabled: true,
        kempston_enabled: false,
        mouse_enabled: false,
        load_default_rom: true,
    };

    info!("Initialize emulator");
    const MAX_FRAME_DURATION: core::time::Duration = core::time::Duration::from_millis(0);

    let mut emulator: Emulator<host::Esp32Host> =
        match Emulator::new(settings, host::Esp32HostContext {}) {
            Ok(emulator) => emulator,
            Err(err) => {
                error!("Error creating emulator: {:?}", err);
                return;
            }
        };

    info!("Loading tape");
    let tape_bytes = include_bytes!("../../data/hello.tap");
    let tape_asset = FileAsset::new(tape_bytes);
    let _ = emulator.load_tape(rustzx_core::host::Tape::Tap(tape_asset));

    info!("Entering emulator loop");
    let mut last_modifier: u8 = 0;

    loop {
        match emulator.emulate_frames(MAX_FRAME_DURATION) {
            Ok(_) => {
                let framebuffer = emulator.screen_buffer();
                if let (Some(top_left), Some(bottom_right)) = (
                    framebuffer.bounding_box_top_left,
                    framebuffer.bounding_box_bottom_right,
                ) {
                    // let width = bottom_right.0 - top_left.0 + 1; // Calculate width
                    // let height = bottom_right.1 - top_left.1 + 1; // Calculate height
                    // debug!("Bounding box: {:?} {:?}", top_left, bottom_right);
                    // debug!("Bounding box size:  {}", width * height);
                    let pixel_iterator = framebuffer.get_region_pixel_iter(top_left, bottom_right);
                    let _ = display.set_pixels(
                        top_left.0 as u16 + SCREEN_OFFSET_X,
                        top_left.1 as u16 + SCREEN_OFFSET_Y,
                        bottom_right.0 as u16 + SCREEN_OFFSET_X,
                        bottom_right.1 as u16 + SCREEN_OFFSET_Y,
                        pixel_iterator,
                    );
                }
                emulator.reset_bounding_box();
            }
            _ => {
                error!("Emulation of frame failed");
            }
        }

        // Read 3 bytes from PIPE if available
        if PIPE.len() >= 3 {
            let mut bytes = [0u8; 3];
            let bytes_read = PIPE.read(&mut bytes).await;
            info!("Bytes read from pipe: {}", bytes_read);
            let (key_state, modifier, key_code) = (bytes[0], bytes[1], bytes[2]);

            // USB Keyaboards send a key up event with modifier 0 when a modifier key is released
            // We need to keep track of the last modifier key pressed to know if we should send a key up event
            if (key_state == 1) && (modifier == 0) {
                handle_key_event(key_state, last_modifier, key_code, &mut emulator);
            } else {
                handle_key_event(key_state, modifier, key_code, &mut emulator);
                last_modifier = modifier;
            }
        }

        Timer::after(Duration::from_millis(5)).await;
    }
}
