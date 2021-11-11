//#![feature(backtrace)]

use std::rc::Rc;
use std::{thread, time::*};

use anyhow::*;
use log::*;

use embedded_svc::anyerror::*;

use esp_idf_hal::prelude::*;

use esp_idf_sys;

use embedded_graphics::prelude::*;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings};

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

    emulate_zx(display::create!(peripherals)?, display::color_conv!())
}

fn emulate_zx<D>(display: D, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> Result<()>
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
        emulation_mode: EmulationMode::Max,
        tape_fastload_enabled: true,
        kempston_enabled: false,
        mouse_enabled: false,
        load_default_rom: true,
    };

    let mut emulator: Emulator<host::Esp32Host<D>> = Emulator::new(
        settings,
        host::Esp32HostContext {
            display: Rc::new(display),
            color_conv,
        },
    )
    .map_err(AnyError::into)?;

    info!("Entering emulator loop");

    loop {
        const MAX_FRAME_TIME: Duration = Duration::from_millis(100);

        let emulator_dt = emulator
            .emulate_frames(MAX_FRAME_TIME)
            .map_err(AnyError::into)?;

        trace!("loop: {}", emulator_dt.as_millis());

        // Yield
        thread::sleep(Duration::from_secs(0));
    }
}
