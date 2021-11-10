#![allow(unused_imports)]
#![allow(clippy::single_component_path_imports)]
//#![feature(backtrace)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Condvar, Mutex};
use std::{cell::RefCell, env, sync::atomic::*, sync::Arc, thread, time::*};

use anyhow::*;
use log::*;

use url;

use smol;

use embedded_svc::anyerror::*;
use embedded_svc::eth;
use embedded_svc::eth::Eth;
use embedded_svc::httpd::registry::*;
use embedded_svc::httpd::*;
use embedded_svc::io;
use embedded_svc::ipv4;
use embedded_svc::ping::Ping;
use embedded_svc::wifi::*;

use esp_idf_svc::eth::*;
use esp_idf_svc::httpd as idf;
use esp_idf_svc::httpd::ServerRegistry;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;

use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_hal::ulp;

use esp_idf_sys;
use esp_idf_sys::esp;

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;

use ili9341;
use ssd1306;
use ssd1306::mode::DisplayConfig;
use st7789;

mod host;
mod app;
use crate::{
    host::{AppHost, AppHostContext},
};
// host::{AppHost, AppHostContext, DetectedFileKind, FileAsset},
use rustzx_core::{
    zx::constants::{
        CANVAS_HEIGHT, CANVAS_WIDTH, CANVAS_X, CANVAS_Y, FPS, SCREEN_HEIGHT, SCREEN_WIDTH,
    },
    Emulator,
    RustzxSettings,
    EmulationMode,
    zx::machine::ZXMachine
};
use host::frame_buffer::{RgbaFrameBuffer};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::Rgb565,
    prelude::*,
};

#[allow(dead_code)]
#[cfg(not(feature = "qemu"))]
const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
#[allow(dead_code)]
#[cfg(not(feature = "qemu"))]
const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");

#[cfg(esp32s2)]
include!(env!("EMBUILD_GENERATED_SYMBOLS_FILE"));

#[cfg(esp32s2)]
const ULP: &[u8] = include_bytes!(env!("EMBUILD_GENERATED_BIN_FILE"));




fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    test_print();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Get backtraces from anyhow; only works for Xtensa arch currently
    #[cfg(arch = "xtensa")]
    env::set_var("RUST_BACKTRACE", "1");

    #[allow(unused)]
    let sys_loop_stack = Arc::new(EspSysLoopStack::new()?);
    #[allow(unused)]
    let default_nvs = Arc::new(EspDefaultNvs::new()?);
    #[allow(unused)]
    let peripherals = Peripherals::take().unwrap();

    #[allow(unused)]
    let pins = peripherals.pins;

    println!("Initializing display");
    let backlight= pins.gpio9;
    let dc=pins.gpio4;
    let rst=pins.gpio8;
    let spi= peripherals.spi3;
    let sclk= pins.gpio6;
    let sdo=pins.gpio7;
    let cs=pins.gpio5;
 
    let config = <spi::config::Config as Default>::default()
        .baudrate(80.MHz().into())
        .bit_order(spi::config::BitOrder::MSBFirst);
 
    let mut backlight = backlight.into_output()?;
    backlight.set_high()?;
 
    println!("Initializing SPI");
    let di = SPIInterfaceNoCS::new(
        spi::Master::<spi::SPI3, _, _, _, _>::new(
            spi,
            spi::Pins {
                sclk,
                sdo,
                sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None,
                cs: Some(cs),
            },
            config,
        )?,
        dc.into_output()?,
    );
 
    let reset = rst.into_output()?;
 
    println!("Initializing display driver");
    let mut display = st7789::ST7789::new(di, reset, 240, 240);

    AnyError::<st7789::Error<_>>::wrap(|| {
        display.init(&mut delay::Ets)?;
        display.set_orientation(st7789::Orientation::Landscape)?;
 
        println!("Clearing display to red color");
        display.clear(Rgb565::RED.into())
 
    });

    println!("Spawning emulator thread");

    let settings = RustzxSettings {
        machine: ZXMachine::Sinclair48K,
        emulation_mode: EmulationMode::Max,
        tape_fastload_enabled: true,
        kempston_enabled: false,
        mouse_enabled: false,
        load_default_rom: true,


   };
   println!("Creating emulator");
   let mut emulator:Emulator<AppHost> = Emulator::new(settings, AppHostContext)
   .map_err(|e| anyhow!("Failed to construct emulator: {}", e))?;
   println!("Loading initial screen buffer");
   let data = emulator.screen_buffer().rgba_data();

   const MAX_FRAME_TIME: Duration = Duration::from_millis(100);
   AnyError::<st7789::Error<_>>::wrap(|| {


       println!("Sending image to display");
       let raw_image = ImageRaw::<Rgb565>::new(data, 256);

       let image = Image::new(&raw_image, Point::zero());
       image.draw(&mut display)
       // display.draw(data);
   });

//    thread::spawn(move||{
   println!("Entering emulator loop");
   loop {
       let emulator_dt = emulator
       .emulate_frames(MAX_FRAME_TIME)
       .map_err(|e| anyhow!("Emulation step failed: {:#?}", e))?;
       println!("loop: {}", emulator_dt.as_millis());

       AnyError::<st7789::Error<_>>::wrap(|| {
           let data = emulator.screen_buffer().rgba_data();
           let raw_image = ImageRaw::<Rgb565>::new(data, 256);

           let image = Image::new(&raw_image, Point::zero());
           image.draw(&mut display)
           // display.draw(data);
       });
       

   }
//    println!("Leaving emulator loop");
// });

    // let mut children = vec![];
    // children.push(thread::spawn(|| run_emulator().unwrap()));
    // thread::spawn(|| run_emulator(display));
    // for child in children {
    //     // Wait for the thread to finish. Returns a result.
    //     let _ = child.join();
    // }
    // println!("No emulator thread left");

    // Ok(())
}

#[allow(clippy::vec_init_then_push)]
fn test_print() {
    // Start simple
    println!("Hello from ZX!");

    // Check collections
    let mut children = vec![];

    children.push("foo");
    children.push("bar");
    println!("More complex print {:?}", children);
}




#[allow(dead_code)]
fn led_draw<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget + Dimensions,
    D::Color: From<Rgb565>,
{
    display.clear(Rgb565::BLACK.into())?;

    Rectangle::new(display.bounding_box().top_left, display.bounding_box().size)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLUE.into())
                .stroke_color(Rgb565::YELLOW.into())
                .stroke_width(1)
                .build(),
        )
        .draw(display)?;

    Text::new(
        "Hello Rust!",
        Point::new(10, (display.bounding_box().size.height - 10) as i32 / 2),
        MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
    )
    .draw(display)?;

    info!("LED rendering done");

    Ok(())
}

#[allow(unused_variables)]
fn httpd(mutex: Arc<(Mutex<Option<u32>>, Condvar)>) -> Result<idf::Server> {
    let server = idf::ServerRegistry::new()
        .at("/")
        .get(|_| Ok("Hello from Rust!".into()))?
        .at("/foo")
        .get(|_| bail!("Boo, something happened!"))?
        .at("/bar")
        .get(|_| {
            Response::new(403)
                .status_message("No permissions")
                .body("You have no permissions to access this page".into())
                .into()
        })?;

    #[cfg(esp32s2)]
    let server = httpd_ulp_endpoints(server, mutex)?;

    server.start(&Default::default())
}

#[cfg(esp32s2)]
fn httpd_ulp_endpoints(
    server: ServerRegistry,
    mutex: Arc<(Mutex<Option<u32>>, Condvar)>,
) -> Result<ServerRegistry> {
    server
        .at("/ulp")
        .get(|_| {
            Ok(r#"
            <doctype html5>
            <html>
                <body>
                    <form method = "post" action = "/ulp_start" enctype="application/x-www-form-urlencoded">
                        Connect a LED to ESP32-S2 GPIO <b>Pin 04</b> and GND.<br>
                        Blink it with ULP <input name = "cycles" type = "text" value = "10"> times
                        <input type = "submit" value = "Go!">
                    </form>
                </body>
            </html>
            "#.into())
        })?
        .at("/ulp_start")
        .post(move |mut request| {
            let body = request.as_bytes()?;

            let cycles = url::form_urlencoded::parse(&body)
                .filter(|p| p.0 == "cycles")
                .map(|p| str::parse::<u32>(&p.1).map_err(Error::msg))
                .next()
                .ok_or(anyhow!("No parameter cycles"))??;

            let mut wait = mutex.0.lock().unwrap();

            *wait = Some(cycles);
            mutex.1.notify_one();

            Ok(format!(
                r#"
                <doctype html5>
                <html>
                    <body>
                        About to sleep now. The ULP chip should blink the LED {} times and then wake me up. Bye!
                    </body>
                </html>
                "#,
                cycles).to_owned().into())
        })
}

#[cfg(esp32s2)]
fn start_ulp(cycles: u32) -> Result<()> {
    use esp_idf_hal::ulp;

    unsafe {
        esp!(esp_idf_sys::ulp_riscv_load_binary(
            ULP.as_ptr(),
            ULP.len() as _
        ))?;
        info!("RiscV ULP binary loaded successfully");

        // Once started, the ULP will wakeup every 5 minutes
        // TODO: Figure out how to disable ULP timer-based wakeup completely, with an ESP-IDF call
        ulp::enable_timer(false);

        info!("RiscV ULP Timer configured");

        info!(
            "Default ULP LED blink cycles: {}",
            core::ptr::read_volatile(CYCLES as *mut u32)
        );

        core::ptr::write_volatile(CYCLES as *mut u32, cycles);
        info!(
            "Sent {} LED blink cycles to the ULP",
            core::ptr::read_volatile(CYCLES as *mut u32)
        );

        esp!(esp_idf_sys::ulp_riscv_run())?;
        info!("RiscV ULP started");

        esp!(esp_idf_sys::esp_sleep_enable_ulp_wakeup())?;
        info!("Wakeup from ULP enabled");

        // Wake up by a timer in 60 seconds
        info!("About to get to sleep now. Will wake up automatically either in 1 minute, or once the ULP has done blinking the LED");
        esp_idf_sys::esp_deep_sleep(Duration::from_secs(60).as_micros() as u64);
    }

    Ok(())
}

#[cfg(not(feature = "qemu"))]
#[allow(dead_code)]
fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    info!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    info!("Wifi configuration set, about to get status");

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Started(ApIpStatus::Done),
    ) = status
    {
        info!("Wifi connected");

        ping(&ip_settings)?;
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

#[cfg(any(feature = "qemu", feature = "w5500", feature = "ip101"))]
fn eth_configure<HW>(mut eth: Box<EspEth<HW>>) -> Result<Box<EspEth<HW>>> {
    info!("Eth created");

    eth.set_configuration(&eth::Configuration::Client(Default::default()))?;

    info!("Eth configuration set, about to get status");

    let status = eth.get_status();

    if let eth::Status::Started(eth::ConnectionStatus::Connected(eth::IpStatus::Done(Some(
        ip_settings,
    )))) = status
    {
        info!("Eth connected");

        ping(&ip_settings)?;
    } else {
        bail!("Unexpected Eth status: {:?}", status);
    }

    Ok(eth)
}

fn ping(ip_settings: &ipv4::ClientSettings) -> Result<()> {
    info!("About to do some pings for {:?}", ip_settings);

    let ping_summary =
        ping::EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
    if ping_summary.transmitted != ping_summary.received {
        bail!(
            "Pinging gateway {} resulted in timeouts",
            ip_settings.subnet.gateway
        );
    }

    info!("Pinging done");

    Ok(())
}

#[cfg(not(feature = "qemu"))]
#[cfg(esp_idf_config_lwip_ipv4_napt)]
fn enable_napt(wifi: &mut EspWifi) -> Result<()> {
    wifi.with_router_netif_mut(|netif| netif.unwrap().enable_napt(true));

    info!("NAPT enabled on the WiFi SoftAP!");

    Ok(())
}

// Kaluga needs customized screen orientation commands
// (not a surprise; quite a few ILI9341 boards need these as evidences in the TFT_eSPI & lvgl ESP32 C drivers)
pub enum KalugaOrientation {
    Portrait,
    PortraitFlipped,
    Landscape,
    LandscapeFlipped,
}

impl ili9341::Mode for KalugaOrientation {
    fn mode(&self) -> u8 {
        match self {
            Self::Portrait => 0,
            Self::Landscape => 0x20 | 0x40,
            Self::PortraitFlipped => 0x80 | 0x40,
            Self::LandscapeFlipped => 0x80 | 0x20,
        }
    }

    fn is_landscape(&self) -> bool {
        matches!(self, Self::Landscape | Self::LandscapeFlipped)
    }
}
