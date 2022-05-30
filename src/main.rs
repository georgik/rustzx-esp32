//#![feature(backtrace)]

use std::{thread, time::*};

use anyhow::*;
use log::*;

use esp_idf_hal::prelude::*;

use esp_idf_sys;

use embedded_graphics::prelude::*;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings, zx::keys::ZXKey};
mod display;
mod host;

use std::sync::Arc;
use embedded_svc::wifi::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;
use std::result::Result::Ok;


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

fn handle_client( mut stream: TcpStream) -> u8 {
    println!("Connected");

    let mut rx_bytes = [0u8; 1];
    // Read from the current data in the TcpStream
    stream.read(&mut rx_bytes).unwrap();
    stream.write(&rx_bytes).unwrap();

    rx_bytes[0]
    // 0
}

const SSID: &str = "";
const PASS: &str = "";

#[allow(dead_code)]
fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> anyhow::Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        password: PASS.into(),
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

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    // accept connections and process them serially
    // for stream in listener.incoming() {
    //     handle_client(stream?);
    // }
    loop {
        const MAX_FRAME_DURATION: Duration = Duration::from_millis(0);

        let duration = emulator
            .emulate_frames(MAX_FRAME_DURATION);

        // info!("Rendering 60 frames took {}ms", duration.as_millis().unwrap());

        // TODO: Screen should be constantly updated from within the emulation cycle, using multithreading
        emulator
            .screen_buffer()
            .blit(&mut display, color_conv);

      for stream in listener.incoming() {
          match stream {
              Ok(stream) => {
                  let key = handle_client(stream);
                  println!("Key: {}", key);
                  match key {
                    0x41 => { emulator.send_key(ZXKey::A, true); }
                    0x42 => { emulator.send_key(ZXKey::B, true); }
                    0x43 => { emulator.send_key(ZXKey::C, true); }
                    0x44 => { emulator.send_key(ZXKey::D, true); }
                    0x45 => { emulator.send_key(ZXKey::E, true); }
                    _ => { println!("unknown key mapping"); }
                  }
            emulator
            .emulate_frames(MAX_FRAME_DURATION);
                  emulator
            .screen_buffer()
            .blit(&mut display, color_conv);
                  match key {
                    0x41 => { emulator.send_key(ZXKey::A, false); }
                    0x42 => { emulator.send_key(ZXKey::B, false); }
                    0x43 => { emulator.send_key(ZXKey::C, false); }
                    0x44 => { emulator.send_key(ZXKey::D, false); }
                    0x45 => { emulator.send_key(ZXKey::E, false); }
                    _ => { println!("unknown key mapping"); }
                  }
                }

              Err(e) => {}
          }
          emulator
          .emulate_frames(MAX_FRAME_DURATION);
                emulator
          .screen_buffer()
          .blit(&mut display, color_conv);

      }


        // for stream in listener.incoming() {
        // match stream {
        //     Ok(stream) => {
        //         let key_value = handle_client(stream);
        //         if key_value != 0 {

        //         }
        //     }
        //     _ => { println!("nop");}
        // }
      // }
        // Yield
        //thread::sleep(Duration::from_secs(0));
    }
}
