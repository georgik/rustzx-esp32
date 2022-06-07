//#![feature(backtrace)]

use std::{thread, time::*};

use anyhow::*;
use log::*;

use esp_idf_hal::prelude::*;

use esp_idf_sys;

use embedded_graphics::prelude::*;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings, zx::keys::ZXKey, zx::keys::CompoundKey};
mod display;
mod host;

use std::sync::Arc;
use embedded_svc::wifi::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;
use std::io::Write;
use std::result::Result::Ok;

// Fonts: https://docs.rs/embedded-graphics/0.7.1/embedded_graphics/mono_font/index.html
use embedded_graphics::mono_font::{ascii::FONT_8X13, MonoTextStyle};
use embedded_graphics::text::*;

/// This configuration is picked up at compile time by `build.rs` from the
/// file `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

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

use std::sync::mpsc::{channel, Sender, Receiver};

fn handle_client(mut stream: TcpStream, tx:Sender<u8>) {
    let mut data = [0 as u8; 256]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            for n in 0..size {
                println!("Sending to queue: {}", data[n]);
                tx.send(data[n]).unwrap();
            }
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

// fn handle_client( mut stream: TcpStream) -> u8 {
//     println!("Connected");

//     let mut rx_bytes = [0u8; 1];
//     // Read from the current data in the TcpStream
//     stream.read(&mut rx_bytes).unwrap();
//     stream.write(&rx_bytes).unwrap();

//     rx_bytes[0]
//     // 0
// }

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

    println!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    info!("to get status");
    let status = wifi.get_status();

    info!("got status)");
    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            _ip_settings,
        ))),
        _,
    ) = status
    {
        println!("Wifi connected");
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}


pub enum Event {
    NoEvent,
    ZXKey(ZXKey, bool),
    ZXKeyWithModifier(ZXKey, ZXKey, bool),
    CompoundKey(CompoundKey, bool),
    // Kempston(KempstonKey, bool),
    // Sinclair(SinclairJoyNum, SinclairKey, bool),
    // MouseMove { x: i8, y: i8 },
    // MouseButton(KempstonMouseButton, bool),
    // MouseWheel(KempstonMouseWheelDirection),
    // SwitchFrameTrace,
    // ChangeJoyKeyboardLayer(bool),
    // ChangeSpeed(EmulationMode),
    // InsertTape,
    // StopTape,
    // QuickSave,
    // QuickLoad,
    // OpenFile(PathBuf),
    // Exit,
}

/// returns ZX Spectum key form scancode of None if not found
fn ascii_code_to_zxkey(ascii_code: u8, pressed: bool) -> Option<Event> {
    let zxkey_event = match ascii_code {
        // Control keys
        0x0A => Some(ZXKey::Enter),
        // 0x13 => Some(ZXKey::Enter),
        // Temporary Enter
        // 0x40 => Some(ZXKey::Enter),

        0x20 => Some(ZXKey::Space),

        // Numbers 0-9
        0x30 => Some(ZXKey::N0),
        0x31 => Some(ZXKey::N1),
        0x32 => Some(ZXKey::N2),
        0x33 => Some(ZXKey::N3),
        0x34 => Some(ZXKey::N4),
        0x35 => Some(ZXKey::N5),
        0x36 => Some(ZXKey::N6),
        0x37 => Some(ZXKey::N7),
        0x38 => Some(ZXKey::N8),
        0x39 => Some(ZXKey::N9),

        // Lower-case letters - a-z
        0x61 => Some(ZXKey::A),
        0x62 => Some(ZXKey::B),
        0x63 => Some(ZXKey::C),
        0x64 => Some(ZXKey::D),
        0x65 => Some(ZXKey::E),
        0x66 => Some(ZXKey::F),
        0x67 => Some(ZXKey::G),
        0x68 => Some(ZXKey::H),
        0x69 => Some(ZXKey::I),
        0x6A => Some(ZXKey::J),
        0x6B => Some(ZXKey::K),
        0x6C => Some(ZXKey::L),
        0x6D => Some(ZXKey::M),
        0x6E => Some(ZXKey::N),
        0x6F => Some(ZXKey::O),
        0x70 => Some(ZXKey::P),
        0x71 => Some(ZXKey::Q),
        0x72 => Some(ZXKey::R),
        0x73 => Some(ZXKey::S),
        0x74 => Some(ZXKey::T),
        0x75 => Some(ZXKey::U),
        0x76 => Some(ZXKey::V),
        0x77 => Some(ZXKey::W),
        0x78 => Some(ZXKey::X),
        0x79 => Some(ZXKey::Y),
        0x7A => Some(ZXKey::Z),

        _ => None,
    };

    return zxkey_event.map(|k| Event::ZXKey(k, pressed))
}


/// returns ZX Spectum key form scancode of None if not found
fn ascii_code_to_modifier(ascii_code: u8, pressed: bool) -> Option<Event> {
    let zxkey_event = match ascii_code {
        // Symbols
        0x21 => Some((ZXKey::SymShift, ZXKey::N1)),    // !
        0x22 => Some((ZXKey::SymShift, ZXKey::P)),     // "
        0x23 => Some((ZXKey::SymShift, ZXKey::N3)),    // #
        0x24 => Some((ZXKey::SymShift, ZXKey::N4)),    // $
        0x25 => Some((ZXKey::SymShift, ZXKey::N5)),    // %
        0x26 => Some((ZXKey::SymShift, ZXKey::N6)),    // &
        0x27 => Some((ZXKey::SymShift, ZXKey::N7)),    // '
        0x28 => Some((ZXKey::SymShift, ZXKey::N8)),    // (
        0x29 => Some((ZXKey::SymShift, ZXKey::N9)),    // )
        0x2A => Some((ZXKey::SymShift, ZXKey::B)),     // *
        0x2B => Some((ZXKey::SymShift, ZXKey::K)),     // +
        0x2C => Some((ZXKey::SymShift, ZXKey::N)),     // ,
        0x2D => Some((ZXKey::SymShift, ZXKey::J)),     // -
        0x2E => Some((ZXKey::SymShift, ZXKey::M)),     // .
        0x2F => Some((ZXKey::SymShift, ZXKey::V)),     // /

        0x3A => Some((ZXKey::SymShift, ZXKey::Z)),     // :
        0x3B => Some((ZXKey::SymShift, ZXKey::O)),     // ;
        0x3C => Some((ZXKey::SymShift, ZXKey::R)),     // <
        0x3D => Some((ZXKey::SymShift, ZXKey::L)),     // =
        0x3E => Some((ZXKey::SymShift, ZXKey::T)),     // >
        0x3F => Some((ZXKey::SymShift, ZXKey::C)),     // ?
        0x40 => Some((ZXKey::SymShift, ZXKey::N2)),    // @

        // Upper-case letters A-Z
        0x41 => Some((ZXKey::Shift, ZXKey::A)),
        0x42 => Some((ZXKey::Shift, ZXKey::B)),
        0x43 => Some((ZXKey::Shift, ZXKey::C)),
        0x44 => Some((ZXKey::Shift, ZXKey::D)),
        0x45 => Some((ZXKey::Shift, ZXKey::E)),
        0x46 => Some((ZXKey::Shift, ZXKey::F)),
        0x47 => Some((ZXKey::Shift, ZXKey::G)),
        0x48 => Some((ZXKey::Shift, ZXKey::H)),
        0x49 => Some((ZXKey::Shift, ZXKey::I)),
        0x4A => Some((ZXKey::Shift, ZXKey::J)),
        0x4B => Some((ZXKey::Shift, ZXKey::K)),
        0x4C => Some((ZXKey::Shift, ZXKey::L)),
        0x4D => Some((ZXKey::Shift, ZXKey::M)),
        0x4E => Some((ZXKey::Shift, ZXKey::N)),
        0x4F => Some((ZXKey::Shift, ZXKey::O)),
        0x50 => Some((ZXKey::Shift, ZXKey::P)),
        0x51 => Some((ZXKey::Shift, ZXKey::Q)),
        0x52 => Some((ZXKey::Shift, ZXKey::R)),
        0x53 => Some((ZXKey::Shift, ZXKey::S)),
        0x54 => Some((ZXKey::Shift, ZXKey::T)),
        0x55 => Some((ZXKey::Shift, ZXKey::U)),
        0x56 => Some((ZXKey::Shift, ZXKey::V)),
        0x57 => Some((ZXKey::Shift, ZXKey::W)),
        0x58 => Some((ZXKey::Shift, ZXKey::X)),
        0x59 => Some((ZXKey::Shift, ZXKey::Y)),
        0x5A => Some((ZXKey::Shift, ZXKey::Z)),

        _ => None,
    };

    zxkey_event.map(|(k, k2)| Event::ZXKeyWithModifier(k, k2, pressed))
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

    // wifi part
    #[allow(unused)]
    let netif_stack = Arc::new(EspNetifStack::new()?);
    #[allow(unused)]
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    #[allow(unused)]
    let default_nvs = Arc::new(EspDefaultNvs::new()?);
    let _wifi = wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )?;

    Text::new(
        "Tcp keyboard socket: :80",
        Point::new(10, 210),
        MonoTextStyle::new(&FONT_8X13, color_conv(ZXColor::White, ZXBrightness::Normal)),
    )
    .draw(&mut display).unwrap();

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    let (tx, rx):(Sender<u8>, Receiver<u8>) = channel();
    let tx_owned = tx.to_owned();
    thread::spawn(move|| {
        // Receive new connection
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let tx_owned = tx_owned.clone();
                    thread::spawn(move|| {
                        // connection succeeded
                        handle_client(stream, tx_owned)
                    });
                }
                Err(e) => {
                }
            }
        }
    });

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
