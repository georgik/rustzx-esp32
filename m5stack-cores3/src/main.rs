#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_display_interface_spi_dma::display_interface_spi_dma;

use static_cell::make_static;

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
    // Uart
};

use embassy_executor::Spawner;
use esp_wifi::esp_now::{EspNow, EspNowError};
use esp_wifi::{initialize, EspWifiInitFor};

use embassy_time::{Duration, Ticker};

use esp_backtrace as _;

// use icm42670::{Address, Icm42670};
use shared_bus::BusManagerSimple;

use log::{info, error};

use mipidsi::models::Model;

use axp2101::{ I2CPowerManagementInterface, Axp2101 };
use aw9523::I2CGpioExpanderInterface;

use esp_bsp::{lcd_gpios, BoardType, DisplayConfig};

use uart_keyboard::uart_receiver;
use esp_now_keyboard::esp_now_receiver;
use emulator::app_loop;

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

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let (lcd_sclk, lcd_mosi, lcd_cs, lcd_miso, lcd_dc, _lcd_backlight, lcd_reset) = lcd_gpios!(BoardType::M5StackCoreS3, io);

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
    let mut delay = Delay::new(&clocks);
    delay.delay_ms(500u32);
    info!("About to initialize the SPI LED driver");

    let timer_group0 = hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timer_group0);

    // ESP-NOW keyboard receiver
    let wifi_timer = hal::timer::TimerGroup::new(peripherals.TIMG1, &clocks).timer0;
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
    // let uart0 = Uart::new(peripherals.UART0, &clocks);
    // spawner.spawn(uart_receiver(uart0)).unwrap();

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

    let di = display_interface_spi_dma::new_no_cs(2 * 256 * 192, spi, lcd_dc);

    let display_config = DisplayConfig::for_board(BoardType::M5StackCoreS3);
    let mut display = match mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(display_config.h_res, display_config.v_res)
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
