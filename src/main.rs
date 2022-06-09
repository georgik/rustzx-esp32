//#![feature(backtrace)]

use std::{time::*};

use anyhow::*;
use log::*;

use esp_idf_hal::prelude::*;

use esp_idf_sys;

use embedded_graphics::prelude::*;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings};
mod display;
mod host;



mod zx_event;

mod ascii_zxkey;

mod tcpstream_keyboard;
use crate::tcpstream_keyboard::{bind_keyboard, Keyboard};

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Get backtraces from anyhow; only works for Xtensa arch currently
    #[cfg(arch = "xtensa")]
    env::set_var("RUST_BACKTRACE", "1");

    let peripherals = Peripherals::take().unwrap();

    emulate_zx(display::create!(peripherals)?, display::color_conv)
}


fn emulate_zx<D>(mut display: D, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> Result<()>
where
    D: DrawTarget + Dimensions + Send + 'static,
    D::Error: core::fmt::Debug,
{
    display
        .clear(color_conv(ZXColor::Blue, ZXBrightness::Normal));

    info!("Creating emulator");

    let settings = RustzxSettings {
        machine: ZXMachine::Sinclair48K,
        emulation_mode: EmulationMode::FrameCount(1),
        tape_fastload_enabled: true,
        kempston_enabled: false,
        mouse_enabled: false,
        load_default_rom: true,
    };

    let mut emulator: Emulator<host::Esp32Host> =
        Emulator::new(settings, host::Esp32HostContext {}).unwrap();

    info!("Entering emulator loop");

    let keyboard = bind_keyboard();
    #[cfg(feature = "tcpstream_keyboard")]
    let rx = keyboard.rx();
    keyboard.spawn_listener();

    let mut key_emulation_delay = 0;
    let mut last_key:u8 = 0;

    loop {
        const MAX_FRAME_DURATION: Duration = Duration::from_millis(0);


        // let mut stats = [0; 1024];
        // unsafe {
        //     // esp_idf_sys::vTaskGetRunTimeStats(stats.as_mut_ptr());
        //     esp_idf_sys::vTaskList(stats.as_mut_ptr());
        //     let message = std::ffi::CStr::from_ptr(stats.as_mut_ptr()).to_str().unwrap().replace("\r","");
        //     println!("{}", message);
        // }


        emulator.emulate_frames(MAX_FRAME_DURATION);
        emulator.screen_buffer()
        .blit(&mut display, color_conv);

        if key_emulation_delay > 0 {
            key_emulation_delay -= 1;
        }

        #[cfg(feature = "tcpstream_keyboard")]
        match rx.try_recv() {
            Ok(key) => {
                if key_emulation_delay > 0 {
                    // It's not possible to process same keys which were entered shortly after each other
                    for frame in 0..key_emulation_delay {
                        println!("Keys received too fast. Running extra emulation frame: {}", frame);
                        emulator.emulate_frames(MAX_FRAME_DURATION);
                    }
                    emulator.screen_buffer()
                    .blit(&mut display, color_conv);
                }

                if key == last_key {
                    // Same key requires bigger delay
                    key_emulation_delay = 6;
                } else {
                    key_emulation_delay = 4;
                }

                last_key = key;

                println!("Key: {} - {}", key, true);
                let mapped_key_down_option = ascii_code_to_zxkey(key, true)
                .or_else(|| ascii_code_to_modifier(key, true));

                let mapped_key_down = match mapped_key_down_option {
                    Some(x) => { x },
                    _ => { Event::NoEvent }
                };

                let mapped_key_up_option = ascii_code_to_zxkey(key, false)
                .or_else(|| ascii_code_to_modifier(key, false));

                let mapped_key_up = match mapped_key_up_option {
                    Some(x) => { x },
                    _ => { Event::NoEvent }
                };

                println!("-> key down");
                match mapped_key_down {
                    Event::ZXKey(k,p) => {
                        println!("-> ZXKey");
                        emulator.send_key(k, p);
                    },
                    Event::ZXKeyWithModifier(k, k2, p) => {
                        println!("-> ZXKeyWithModifier");
                        emulator.send_key(k, p);
                        emulator.send_key(k2, p);
                    }
                    _ => {
                        println!("Unknown key.");
                    }
                }

                println!("-> emulating frame");
                emulator.emulate_frames(MAX_FRAME_DURATION);
                emulator.screen_buffer()
                    .blit(&mut display, color_conv);

                println!("-> key up");
                match mapped_key_up {
                    Event::ZXKey(k,p) => {
                        emulator.send_key(k, p);
                    },
                    Event::ZXKeyWithModifier(k, k2, p) => {
                        emulator.send_key(k, p);
                        emulator.send_key(k2, p);
                    }
                    _ => {}
                }

            },
            _ => {
            }
        }
    }
}
