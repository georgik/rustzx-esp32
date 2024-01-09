#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use spi_dma_displayinterface::spi_dma_displayinterface;

use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    prelude::{Point, RgbColor},
    text::Text,
    Drawable,
};

use hal::{
    clock::{ClockControl, CpuClock},
    dma::DmaPriority,
    embassy,
    gdma::Gdma,
    peripherals::Peripherals,
    prelude::*,
    psram,
    spi::{
        master::{prelude::*, Spi},
        SpiMode,
    },
    Delay,
    Rng,
    IO,
};

use spooky_embedded::{
    embedded_display::{LCD_H_RES, LCD_V_RES},
    // embedded_display::LCD_MEMORY_SIZE,
    // controllers::{accel::AccelMovementController, composites::accel_composite::AccelCompositeController}
};

use esp_backtrace as _;

use esp_wifi::{initialize, EspWifiInitFor};

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings, host::Host};

use log::{info, error, debug};

use embedded_graphics::pixelcolor::Rgb565;

use embedded_hal::digital::v2::OutputPin;

mod host;
mod stopwatch;
mod io;
mod usb_zxkey;
use usb_zxkey::{ usb_code_to_zxkey, usb_code_to_modifier };
mod zx_event;
use zx_event::Event;

use crate::io::FileAsset;

use embassy_executor::Spawner;
use esp_wifi::esp_now::{EspNow, PeerInfo, BROADCAST_ADDRESS};

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

const SCREEN_OFFSET_X: u16 = (320 - 256) / 2;
const SCREEN_OFFSET_Y: u16 = (240 - 192) / 2;

fn init_psram_heap() {
    unsafe {
        ALLOCATOR.init(psram::psram_vaddr_start() as *mut u8, psram::PSRAM_BYTES);
    }
}

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::pipe::Pipe;

// Pipe for transporting keystrokes from ESP-NOW to emulator core
const PIPE_BUF_SIZE: usize = 15;
static PIPE: Pipe<CriticalSectionRawMutex, PIPE_BUF_SIZE> = Pipe::new();

use embassy_time::{Duration, Ticker, Timer};
#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();

    psram::init_psram(peripherals.PSRAM);
    init_psram_heap();

    let system = peripherals.SYSTEM.split();

    esp_println::logger::init_logger_from_env();


    info!("Starting up");
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();
    let embassy_timer = hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0;
    embassy::init(&clocks, embassy_timer);
    spawner.spawn(app_loop()).unwrap();
    spawner.spawn(esp_now_receiver()).unwrap();
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        info!("Tick");
        ticker.next().await;
    }

}

const ZX_BLACK: Rgb565 = Rgb565::BLACK;
const ZX_BRIGHT_BLUE: Rgb565 = Rgb565::new(0, 0, Rgb565::MAX_B);
const ZX_BRIGHT_RED: Rgb565 = Rgb565::new(Rgb565::MAX_R, 0, 0);
const ZX_BRIGHT_PURPLE: Rgb565 = Rgb565::new(Rgb565::MAX_R, 0, Rgb565::MAX_B);
const ZX_BRIGHT_GREEN: Rgb565 = Rgb565::new(0, Rgb565::MAX_G, 0);
const ZX_BRIGHT_CYAN: Rgb565 = Rgb565::new(0, Rgb565::MAX_G, Rgb565::MAX_B);
const ZX_BRIGHT_YELLOW: Rgb565 = Rgb565::new(Rgb565::MAX_R, Rgb565::MAX_G, 0);
const ZX_BRIGHT_WHITE: Rgb565 = Rgb565::WHITE;

const ZX_NORMAL_BLUE: Rgb565 = Rgb565::new(0, 0, Rgb565::MAX_B / 2);
const ZX_NORMAL_RED: Rgb565 = Rgb565::new(Rgb565::MAX_R / 2, 0, 0);
const ZX_NORMAL_PURPLE: Rgb565 = Rgb565::new(Rgb565::MAX_R / 2, 0, Rgb565::MAX_B / 2);
const ZX_NORMAL_GREEN: Rgb565 = Rgb565::new(0, Rgb565::MAX_G / 2, 0);
const ZX_NORMAL_CYAN: Rgb565 = Rgb565::new(0, Rgb565::MAX_G / 2, Rgb565::MAX_B / 2);
const ZX_NORMAL_YELLOW: Rgb565 = Rgb565::new(Rgb565::MAX_R / 2, Rgb565::MAX_G / 2, 0);
const ZX_NORMAL_WHITE: Rgb565 = Rgb565::new(Rgb565::MAX_R / 2, Rgb565::MAX_G / 2, Rgb565::MAX_B / 2);

fn color_conv(color: &ZXColor, brightness: ZXBrightness) -> Rgb565 {
    match (color, brightness) {
        (ZXColor::Black, _) => ZX_BLACK,

        // Bright Colors
        (ZXColor::Blue, ZXBrightness::Bright) => ZX_BRIGHT_BLUE,
        (ZXColor::Red, ZXBrightness::Bright) => ZX_BRIGHT_RED,
        (ZXColor::Purple, ZXBrightness::Bright) => ZX_BRIGHT_PURPLE,
        (ZXColor::Green, ZXBrightness::Bright) => ZX_BRIGHT_GREEN,
        (ZXColor::Cyan, ZXBrightness::Bright) => ZX_BRIGHT_CYAN,
        (ZXColor::Yellow, ZXBrightness::Bright) => ZX_BRIGHT_YELLOW,
        (ZXColor::White, ZXBrightness::Bright) => ZX_BRIGHT_WHITE,

        // Normal Colors
        (ZXColor::Blue, ZXBrightness::Normal) => ZX_NORMAL_BLUE,
        (ZXColor::Red, ZXBrightness::Normal) => ZX_NORMAL_RED,
        (ZXColor::Purple, ZXBrightness::Normal) => ZX_NORMAL_PURPLE,
        (ZXColor::Green, ZXBrightness::Normal) => ZX_NORMAL_GREEN,
        (ZXColor::Cyan, ZXBrightness::Normal) => ZX_NORMAL_CYAN,
        (ZXColor::Yellow, ZXBrightness::Normal) => ZX_NORMAL_YELLOW,
        (ZXColor::White, ZXBrightness::Normal) => ZX_NORMAL_WHITE,
    }
}

fn handle_key_event<H: Host>(key_state: u8, modifier: u8, key_code:u8, emulator: &mut Emulator<H>) {
    let is_pressed = key_state == 0;
    if let Some(mapped_key) = usb_code_to_zxkey(key_code, is_pressed).or_else(|| usb_code_to_modifier(key_code, is_pressed)) {
        match mapped_key {
            Event::ZXKey(k, p) => {
                debug!("-> ZXKey");
                emulator.send_key(k, p);
            },
            Event::NoEvent => {
                error!("Key not implemented");
            },
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


const ESP_NOW_PAYLOAD_INDEX: usize = 20;

#[embassy_executor::task]
async fn esp_now_receiver() {
    info!("ESP-NOW receiver task");
    let peripherals = unsafe { Peripherals::steal() };
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();
    let wifi_timer = hal::timer::TimerGroup::new(peripherals.TIMG1, &clocks).timer0;
    let rng = Rng::new(peripherals.RNG);
    let radio_clock_control = system.radio_clock_control;


    let wifi = peripherals.WIFI;

    let init = initialize(
        EspWifiInitFor::Wifi,
        wifi_timer,
        rng,
        radio_clock_control,
        &clocks,
    );

    match init {
        Ok(init) => {
            info!("ESP-NOW init");
            let mut esp_now = EspNow::new(&init, wifi);
            match esp_now {
                Ok(mut esp_now) => {
                    let peer_info = PeerInfo {
                        // Specify a unique peer address here (replace with actual address)
                        peer_address: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
                        lmk: None,
                        channel: Some(1), // Specify the channel if known
                        encrypt: false, // Set to true if encryption is needed
                    };

                    // Check if the peer already exists
                    if !esp_now.peer_exists(&peer_info.peer_address) {
                        info!("Adding peer");
                        match esp_now.add_peer(peer_info) {
                            Ok(_) => info!("Peer added"),
                            Err(e) => error!("Peer add error: {:?}", e),
                        }
                    } else {
                        info!("Peer already exists, not adding");
                    }

                    loop {
                        let received_data = esp_now.receive();
                        match received_data {
                            Some(data) => {
                                let bytes = data.data;
                                info!("Key code received over ESP-NOW: state = {:?}, modifier = {:?}, key = {:?}", bytes[ESP_NOW_PAYLOAD_INDEX], bytes[ESP_NOW_PAYLOAD_INDEX + 1], bytes[ESP_NOW_PAYLOAD_INDEX + 2]);
                                let bytes_written = PIPE.write(&[bytes[ESP_NOW_PAYLOAD_INDEX], bytes[ESP_NOW_PAYLOAD_INDEX + 1], bytes[ESP_NOW_PAYLOAD_INDEX + 2]]).await;
                                if bytes_written != 3 {
                                    error!("Failed to write to pipe");
                                    break;
                                }
                            }
                            None => {
                                //error!("ESP-NOW receive error");
                            }
                        }
                        Timer::after(Duration::from_millis(5)).await;
                    }
                }
                Err(e) => error!("ESP-NOW initialization error: {:?}", e),
            }
        }
        Err(e) => error!("WiFi initialization error: {:?}", e),
    }
}


#[embassy_executor::task]
async fn app_loop()
 //-> Result<(), core::fmt::Error>
{

    Timer::after(Duration::from_millis(1500)).await;

    let peripherals = unsafe { Peripherals::steal() };
    let system = peripherals.SYSTEM.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();


    let lcd_sclk = io.pins.gpio7;
    let lcd_mosi = io.pins.gpio6;
    let lcd_cs = io.pins.gpio5;
    let lcd_miso = io.pins.gpio2; // random unused pin
    let lcd_dc = io.pins.gpio4.into_push_pull_output();
    let mut lcd_backlight = io.pins.gpio45.into_push_pull_output();
    let lcd_reset = io.pins.gpio48.into_push_pull_output();

    let dma = Gdma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    let mut descriptors = [0u32; 8 * 3];
    let mut rx_descriptors = [0u32; 8 * 3];


    let mut delay = Delay::new(&clocks);

    // delay.delay_ms(500u32);
    info!("About to initialize the SPI LED driver");

    let spi = Spi::new(
        peripherals.SPI3,
        40u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    ).with_pins(
        Some(lcd_sclk),
        Some(lcd_mosi),
        Some(lcd_miso),
        Some(lcd_cs),
    ).with_dma(dma_channel.configure(
        false,
        &mut descriptors,
        &mut rx_descriptors,
        DmaPriority::Priority0,
    ));

    Timer::after(Duration::from_millis(500)).await;

    let _ = lcd_backlight.set_high();
    //https://github.com/m5stack/M5CoreS3/blob/main/src/utility/Config.h#L8
    let di = spi_dma_displayinterface::new_no_cs(2*256*192, spi, lcd_dc);

    let mut display = match mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(LCD_H_RES, LCD_V_RES)
        .with_orientation(mipidsi::Orientation::PortraitInverted(false))
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .init(&mut delay, Some(lcd_reset))
    {
        Ok(display) => display,
        Err(_e) => {
            // Handle the error and possibly exit the application
            panic!("Display initialization failed");
        }
    };

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

    loop {
        match emulator.emulate_frames(MAX_FRAME_DURATION) {
            Ok(_) => {
                let framebuffer = emulator.screen_buffer();
                if let (Some(top_left), Some(bottom_right)) = (framebuffer.bounding_box_top_left, framebuffer.bounding_box_bottom_right) {
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
                        pixel_iterator);
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
            handle_key_event(bytes[0], bytes[1], bytes[2], &mut emulator);
        }

        Timer::after(Duration::from_millis(5)).await;
    }
}
