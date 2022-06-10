//#![feature(backtrace)]

use std::{time::*};

use anyhow::*;
use log::*;

use esp_idf_hal::prelude::*;

use esp_idf_sys;
use std::sync::Arc;
use embedded_svc::wifi::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;

use embedded_graphics::prelude::*;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings};
mod display;
mod host;

use std::result::Result::Ok;
// Fonts: https://docs.rs/embedded-graphics/0.7.1/embedded_graphics/mono_font/index.html
use embedded_graphics::mono_font::{ascii::FONT_8X13, MonoTextStyle};
use embedded_graphics::text::*;

mod zx_event;
// #[cfg(feature = "tcpstream_keyboard")]
use zx_event::Event;

mod ascii_zxkey;
// #[cfg(feature = "tcpstream_keyboard")]
use ascii_zxkey::{ascii_code_to_modifier, ascii_code_to_zxkey};

mod tcpstream_keyboard;
// #[cfg(feature = "tcpstream_keyboard")]
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


/// This configuration is picked up at compile time by `build.rs` from the
/// file `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}



#[allow(dead_code)]
fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> anyhow::Result<Box<EspWifi>> {
    let app_config = CONFIG;
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: app_config.wifi_ssid.into(),
        password: app_config.wifi_psk.into(),
        auth_method: AuthMethod::None,
        ..Default::default()
    }))?;

    info!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    info!("to get status");
    let status = wifi.get_status();

    info!("got status)");
    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            ip_settings,
        ))),
        _,
    ) = status
    {
        info!("Wifi connected. IP address: {}", ip_settings.ip);
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
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

    info!("Initialize emulator");
    const MAX_FRAME_DURATION: Duration = Duration::from_millis(0);
    emulator.emulate_frames(MAX_FRAME_DURATION);
    emulator.screen_buffer()
    .blit(&mut display, color_conv);

    info!("Initializing WiFi");
    // wifi part
    #[allow(unused)]
    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    #[allow(unused)]
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    #[allow(unused)]
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());
    #[cfg(feature = "tcpstream_keyboard")]
    let wifi_interface = wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    ).unwrap();

    info!("Binding keyboard");

    #[cfg(feature = "tcpstream_keyboard")]
    let rx = bind_keyboard(23);

    let mut stage = 0;
    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(config))),
        _,
    ) = wifi_interface.get_status()
    {
        match stage {
            0 => {
                let message = format!("Keyboard: {}:23", config.ip);
                println!("{}", message);
                Text::new(
                    message.as_str(),
                    Point::new(10, 210),
                    MonoTextStyle::new(&FONT_8X13, color_conv(ZXColor::White, ZXBrightness::Normal)),
                )
                .draw(&mut display).unwrap();

            }
            _ => {
                println!("WiFi unknown");
            }
        }
    }

    let mut key_emulation_delay = 0;
    let mut last_key:u8 = 0;

    info!("Entering emulator loop");

    loop {


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
                        debug!("Keys received too fast. Running extra emulation frame: {}", frame);
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

                info!("Key: {} - {}", key, true);
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

                debug!("-> key down");
                match mapped_key_down {
                    Event::ZXKey(k,p) => {
                        debug!("-> ZXKey");
                        emulator.send_key(k, p);
                    },
                    Event::ZXKeyWithModifier(k, k2, p) => {
                        debug!("-> ZXKeyWithModifier");
                        emulator.send_key(k, p);
                        emulator.send_key(k2, p);
                    }
                    _ => {
                        debug!("Unknown key.");
                    }
                }

                debug!("-> emulating frame");
                emulator.emulate_frames(MAX_FRAME_DURATION);
                emulator.screen_buffer()
                    .blit(&mut display, color_conv);

                debug!("-> key up");
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
