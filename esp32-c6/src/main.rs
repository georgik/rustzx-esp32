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
    peripherals::Peripherals,
    prelude::*,
    spi::{
        master::{prelude::*, Spi},
        SpiMode,
    },
    Delay,
    Rng,
    IO,
    systimer::SystemTimer,
    Uart
};

use esp_backtrace as _;

use esp_wifi::{initialize, EspWifiInitFor};

use log::{info, error};

use embassy_executor::Spawner;
use esp_wifi::esp_now::{EspNow, EspNowError};

use core::mem::MaybeUninit;

use uart_keyboard::uart_receiver;
use esp_now_keyboard::esp_now_receiver;
use emulator::app_loop;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 280 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

use embassy_time::{Duration, Ticker};

use esp_bsp::{BoardType, lcd_gpios, DisplayConfig};

#[main]
async fn main(spawner: Spawner) -> ! {
    init_heap();

    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock160MHz).freeze();

    esp_println::logger::init_logger_from_env();

    info!("Starting up");
    let timer_group0 = hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timer_group0);

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
    let (lcd_sclk, lcd_mosi, lcd_cs, lcd_miso, lcd_dc, _lcd_backlight, lcd_reset) = lcd_gpios!(BoardType::ESP32C6DevKitC1, io);

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

    let display_config = DisplayConfig::for_board(BoardType::ESP32C6DevKitC1);
    let display = match mipidsi::Builder::ili9341_rgb565(di)
        .with_display_size(display_config.h_res, display_config.v_res)
        .with_orientation(mipidsi::Orientation::Landscape(true))
        .with_color_order(mipidsi::ColorOrder::Rgb)
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
