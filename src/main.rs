//#![feature(backtrace)]

use std::{thread, time::*};

use anyhow::*;
use log::*;

use embedded_svc::anyerror::*;

use esp_idf_hal::prelude::*;

use esp_idf_sys;

use embedded_graphics::prelude::*;

use esp_idf_hal::gpio;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings, zx::keys::ZXKey};

mod display;
mod host;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Get backtraces from anyhow; only works for Xtensa arch currently
    #[cfg(arch = "xtensa")]
    env::set_var("RUST_BACKTRACE", "1");

    let peripherals = Peripherals::take().unwrap();
    let button = peripherals.pins.gpio14.into_input()?;

    emulate_zx(display::create!(peripherals)?, display::color_conv, button)
}

fn emulate_zx<D>(mut display: D, color_conv: fn(ZXColor, ZXBrightness) -> D::Color, button: gpio::Gpio14<gpio::Input>) -> Result<()>
where
    D: DrawTarget + Dimensions + Send + 'static,
    D::Error: core::fmt::Debug,
{
    display
        .clear(color_conv(ZXColor::Red, ZXBrightness::Normal))
        .map_err(AnyError::into)?;

    info!("Creating emulator");
    

    let settings = RustzxSettings {
        machine: ZXMachine::Sinclair48K,
        emulation_mode: EmulationMode::FrameCount(2),
        tape_fastload_enabled: true,
        kempston_enabled: false,
        mouse_enabled: false,
        load_default_rom: true,
    };

    let mut emulator: Emulator<host::Esp32Host> =
        Emulator::new(settings, host::Esp32HostContext {}).map_err(AnyError::into)?;

    info!("Entering emulator loop");

    loop {
        const MAX_FRAME_DURATION: Duration = Duration::from_millis(100);

        let duration = emulator
            .emulate_frames(MAX_FRAME_DURATION)
            .map_err(AnyError::into)?;

        info!("Rendering 1 frames took {}ms", duration.as_millis());

        // TODO: Screen should be constantly updated from within the emulation cycle, using multithreading
        emulator
            .screen_buffer()
            .blit(&mut display, color_conv)
            .map_err(AnyError::into)?;

        info!("Button {}",button.is_high()?);
        let key = ZXKey::J;
        emulator.send_key(key, button.is_high()?);
        // Yield
        //thread::sleep(Duration::from_secs(0));
    }
}
