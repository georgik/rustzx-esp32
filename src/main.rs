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

use esp_idf_svc::eth::*;
use esp_idf_svc::httpd as idf;
use esp_idf_svc::httpd::ServerRegistry;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sysloop::*;

use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;

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




const BUILD_TIME : &str = include!(concat!(env!("OUT_DIR"), "/timestamp.txt"));

fn main() -> Result<()> {
    println!("Build timestamp: {}", BUILD_TIME);
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

        println!("Clearing display");
        display.clear(Rgb565::WHITE.into())
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

       let sub_image = raw_image.sub_image(&Rectangle::new(Point::new(0, 0), Size::new(240, 192)));
       Image::new(&sub_image, Point::new(0, 0)).draw(&mut display)
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
           let sub_image = raw_image.sub_image(&Rectangle::new(Point::new(0, 0), Size::new(240, 192)));
           Image::new(&sub_image, Point::new(0, 0)).draw(&mut display)
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
