#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use spi_dma_displayinterface::spi_dma_displayinterface;
use static_cell::make_static;

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
    i2c,
    // interrupt,
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

use embassy_executor::Spawner;
use esp_wifi::esp_now::{EspNow, PeerInfo};
use esp_wifi::{initialize, EspWifiInitFor};

use embassy_time::{Duration, Ticker, Timer};

use esp_backtrace as _;

// use icm42670::{Address, Icm42670};
use shared_bus::BusManagerSimple;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings, host::Host};

use log::{info, error, debug};

// use core::time::Duration;
use embedded_graphics::pixelcolor::Rgb565;

use display_interface::WriteOnlyDataCommand;
use mipidsi::models::Model;
use embedded_hal::digital::v2::OutputPin;

use axp2101::{ I2CPowerManagementInterface, Axp2101 };
use aw9523::I2CGpioExpanderInterface;

use pc_keyboard::{layouts, HandleControl, ScancodeSet2};

mod host;
mod stopwatch;
mod io;
use usb_zx::{
    usb_zx_key::usb_code_to_zxkey,
    zx_event::Event
};

use crate::io::FileAsset;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

const SCREEN_OFFSET_X: u16 = (320 - 256) / 2;
const SCREEN_OFFSET_Y: u16 = (240 - 192) / 2;

fn init_psram_heap() {
    unsafe {
        ALLOCATOR.init(psram::psram_vaddr_start() as *mut u8, psram::PSRAM_BYTES);
    }
}

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();

    psram::init_psram(peripherals.PSRAM);
    init_psram_heap();

    let system = peripherals.SYSTEM.split();

    esp_println::logger::init_logger_from_env();

    info!("Starting up");
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();

    // I2C
    let sda = io.pins.gpio12;
    let scl = io.pins.gpio11;

    let i2c_bus = i2c::I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        400u32.kHz(),
        &clocks,
    );

    let bus = BusManagerSimple::new(i2c_bus);

    info!("Initializing AXP2101");
    let axp_interface = I2CPowerManagementInterface::new(bus.acquire_i2c());
    let mut axp = Axp2101::new(axp_interface);
    axp.init().unwrap();

    info!("Initializing GPIO Expander");
    let aw_interface = I2CGpioExpanderInterface::new(bus.acquire_i2c());
    let mut aw = aw9523::Aw9523::new(aw_interface);
    aw.init().unwrap();

    // M5Stack CORE 2 - https://docs.m5stack.com/en/core/core2
    // let mut backlight = io.pins.gpio3.into_push_pull_output();
    delay.delay_ms(500u32);
    info!("About to initialize the SPI LED driver");

    let embassy_timer = hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0;
    embassy::init(&clocks, embassy_timer);

    // ESP-NOW keyboard receiver
    let wifi_timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let rng = Rng::new(peripherals.RNG);
    let radio_clock_control = system.radio_clock_control;

    let wifi = peripherals.WIFI;

    let esp_now_init = initialize(
        EspWifiInitFor::Wifi,
        wifi_timer,
        rng,
        radio_clock_control,
        &clocks,
    );

    match esp_now_init {
        Ok(init) => {
            info!("ESP-NOW init");
            let esp_now: Result<EspNow, EspNowError> = EspNow::new(&init, wifi);
            match esp_now {
                Ok(esp_now) => {
                    spawner.spawn(esp_now_receiver(esp_now)).unwrap();
                }
                _ => {
                    error!("ESP-NOW startup error");
                }
            }
        }
        _ => {
            error!("ESP-NOW init error");
        }
    }

    // UART Keyboard receiver
    let uart0 = Uart::new(peripherals.UART0, &clocks);
    spawner.spawn(uart_receiver(uart0)).unwrap();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let (lcd_sclk, lcd_mosi, lcd_cs, lcd_miso, lcd_dc, _lcd_backlight, lcd_reset) = lcd_gpios!("ESP32-C6-DevKitC-1", io);

    let dma = Gdma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    let mut delay = Delay::new(&clocks);

    let descriptors = make_static!([0u32; 8 * 3]);
    let rx_descriptors = make_static!([0u32; 8 * 3]);
    info!("About to initialize the SPI LED driver");

    let spi = Spi::new(
            peripherals.SPI2,
            40u32.MHz(),
            SpiMode::Mode0,
            &clocks
        ).with_pins(
            Some(lcd_sclk),
            Some(lcd_mosi),
            Some(lcd_miso),
            Some(lcd_cs),
        ).with_dma(
            dma_channel.configure(
                false,
                &mut *descriptors,
                &mut *rx_descriptors,
                DmaPriority::Priority0,
        )
    );

    let di = spi_dma_displayinterface::new_no_cs(2 * 256 * 192, spi, lcd_dc);

    let mut display = match mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(320, 240)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut delay, Some(lcd_reset))
    {
        Ok(display) => display,
        Err(_e) => {
            // Handle the error and possibly exit the application
            panic!("Display initialization failed");
        }
    };

    // Main Emulator loop
    spawner.spawn(app_loop(display)).unwrap();

    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        info!("Tick");
        ticker.next().await;
    }

}
