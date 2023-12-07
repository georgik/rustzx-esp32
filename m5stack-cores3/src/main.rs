#![no_std]
#![no_main]

use spi_dma_displayinterface::spi_dma_displayinterface;

use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    prelude::{Point, RgbColor},
    text::Text,
    Drawable,
};

use esp_println::println;
use core::cell::RefCell;

use hal::{
    clock::{ClockControl, CpuClock},
    dma::DmaPriority,
    gdma::Gdma,
    i2c,
    interrupt,
    peripherals::{
        Peripherals,
        Interrupt
    },
    prelude::*,
    spi::{
        master::{prelude::*, Spi},
        SpiMode,
    },
    Delay,
    Rng,
    IO
};

// use spooky_embedded::app::app_loop;

use spooky_embedded::{
    embedded_display::{LCD_H_RES, LCD_V_RES, LCD_MEMORY_SIZE},
    controllers::{accel::AccelMovementController, composites::accel_composite::AccelCompositeController}
};

use esp_backtrace as _;

use icm42670::{Address, Icm42670};
use shared_bus::BusManagerSimple;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings};

use log::{info, error};

use core::time::Duration;
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565
};

use display_interface::WriteOnlyDataCommand;
use mipidsi::models::Model;
use embedded_hal::digital::v2::OutputPin;

use core::mem::MaybeUninit;
use critical_section::Mutex;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 300 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}


#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    // With DMA we have sufficient throughput, so we can clock down the CPU to 80MHz
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock160MHz).freeze();

    esp_println::logger::init_logger_from_env();

    let mut delay = Delay::new(&clocks);

    info!("About to initialize the SPI LED driver");
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let lcd_sclk = io.pins.gpio0;
    let lcd_mosi = io.pins.gpio6;
    let lcd_miso = io.pins.gpio11; // random unused pin
    let lcd_cs = io.pins.gpio5;
    let lcd_dc = io.pins.gpio4.into_push_pull_output();
    let _lcd_backlight = io.pins.gpio1.into_push_pull_output();
    let lcd_reset = io.pins.gpio3.into_push_pull_output();

    let i2c_sda = io.pins.gpio10;
    let i2c_scl = io.pins.gpio8;

    let dma = Gdma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    let mut descriptors = [0u32; 8 * 3];
    let mut rx_descriptors = [0u32; 8 * 3];

    let spi = Spi::new(
        peripherals.SPI2,
        lcd_sclk,
        lcd_mosi,
        lcd_miso,
        lcd_cs,
        60u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    ).with_dma(dma_channel.configure(
        false,
        &mut descriptors,
        &mut rx_descriptors,
        DmaPriority::Priority0,
    ));

    info!("SPI ready");

    let di = spi_dma_displayinterface::new_no_cs(LCD_MEMORY_SIZE, spi, lcd_dc);

    // ESP32-S3-BOX display initialization workaround: Wait for the display to power up.
    // If delay is 250ms, picture will be fuzzy.
    // If there is no delay, display is blank
    delay.delay_ms(500u32);

    let mut display = match mipidsi::Builder::st7789(di)
    .with_display_size(LCD_H_RES, LCD_V_RES)
    .with_orientation(mipidsi::Orientation::Landscape(true))
    .with_color_order(mipidsi::ColorOrder::Rgb)
        .init(&mut delay, Some(lcd_reset)) {
        Ok(display) => display,
        Err(_e) => {
            // Handle the error and possibly exit the application
            panic!("Display initialization failed");
        }
    };

    info!("Initializing...");
    Text::new(
        "Initializing...",
        Point::new(80, 110),
        MonoTextStyle::new(&FONT_8X13, RgbColor::GREEN),
    )
    .draw(&mut display)
    .unwrap();

    info!("Initialized");

    // let i2c = i2c::I2C::new(
    //     peripherals.I2C0,
    //     i2c_sda,
    //     i2c_scl,
    //     2u32.kHz(), // Set just to 2 kHz, it seems that there is an interference on Rust board
    //     &clocks,
    // );

    // info!("I2C ready");

    // let bus = BusManagerSimple::new(i2c);
    // let icm = Icm42670::new(i2c, Address::Primary).unwrap();

    // let mut rng = Rng::new(peripherals.RNG);
    // let mut seed_buffer = [0u8; 32];
    // rng.read(&mut seed_buffer).unwrap();

    // let accel_movement_controller = AccelMovementController::new(icm, 0.2);
    // let demo_movement_controller = spooky_core::demo_movement_controller::DemoMovementController::new(seed_buffer);
    // let movement_controller = AccelCompositeController::new(demo_movement_controller, accel_movement_controller);

    // app_loop( &mut display, seed_buffer, movement_controller);
    let _ = app_loop(&mut display, color_conv);
    loop {}

}


fn color_conv(color: ZXColor, _brightness: ZXBrightness) -> Rgb565 {
    match color {
        ZXColor::Black => Rgb565::BLACK,
        ZXColor::Blue => Rgb565::BLUE,
        ZXColor::Red => Rgb565::RED,
        ZXColor::Purple => Rgb565::MAGENTA,
        ZXColor::Green => Rgb565::GREEN,
        ZXColor::Cyan => Rgb565::CYAN,
        ZXColor::Yellow => Rgb565::YELLOW,
        ZXColor::White => Rgb565::WHITE,
    }
}

mod host;
mod stopwatch;
mod io;
mod spritebuf;
fn app_loop<DI, M, RST>(
    display: &mut mipidsi::Display<DI, M, RST>,
    color_conv: fn(ZXColor, ZXBrightness) -> Rgb565) //-> Result<(), core::fmt::Error>
where
    DI: WriteOnlyDataCommand,
    M: Model<ColorFormat = Rgb565>,
    RST: OutputPin,
{
    // display
    //     .clear(color_conv(ZXColor::Blue, ZXBrightness::Normal))
    //     .map_err(|err| error!("{:?}", err))
    //     .ok();

    info!("Creating emulator");

    let settings = RustzxSettings {
        machine: ZXMachine::Sinclair48K,
        emulation_mode: EmulationMode::FrameCount(1),
        tape_fastload_enabled: true,
        kempston_enabled: false,
        mouse_enabled: false,
        load_default_rom: true,
    };

    info!("Initialize emulator");
    const MAX_FRAME_DURATION: Duration = Duration::from_millis(0);

    let mut emulator: Emulator<host::Esp32Host> =
        match Emulator::new(settings, host::Esp32HostContext {}) {
            Ok(emulator) => emulator,
            Err(err) => {
                error!("Error creating emulator: {:?}", err);
                return;
            }
        };


    // info!("Binding keyboard");

    // #[cfg(feature = "tcpstream_keyboard")]
    // let rx = bind_keyboard(80);

    // #[cfg(feature = "tcpstream_keyboard")]
    // let stage = 0;
    // #[cfg(feature = "tcpstream_keyboard")]
    // if let Status(
    //     ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(config))),
    //     _,
    // ) = wifi_interface.get_status()
    // {
    //     match stage {
    //         0 => {
    //             let message = format!("Keyboard: {}:80", config.ip);
    //             println!("{}", message);
    //             Text::new(
    //                 message.as_str(),
    //                 Point::new(10, 210),
    //                 MonoTextStyle::new(&FONT_8X13, color_conv(ZXColor::White, ZXBrightness::Normal)),
    //             )
    //             .draw(&mut display).unwrap();

    //         }
    //         _ => {
    //             println!("WiFi unknown");
    //         }
    //     }
    // }

    // #[cfg(feature = "tcpstream_keyboard")]
    // let mut key_emulation_delay = 0;
    // #[cfg(feature = "tcpstream_keyboard")]
    // let mut last_key:u8 = 0;

    info!("Entering emulator loop");

    loop {
        info!("Emulating frame");

        // emulator.emulate_frames(MAX_FRAME_DURATION);
        match emulator.emulate_frames(MAX_FRAME_DURATION) {
            Ok(_) => {
                info!("Emulation of frame succeeded");
                let pixel_iterator = emulator.screen_buffer().get_pixel_iter();
                info!("Drawing frame");
                let _ = display.set_pixels(0, 0, 256 - 1, 192, pixel_iterator);
                    // .blit(&mut display, color_conv)
                    // .map_err(|err| error!("{:?}", err))
                    // .ok();
            }
            _ => {
              error!("Emulation of frame failed");
            }
        }

    //     #[cfg(feature = "tcpstream_keyboard")]
    //     if key_emulation_delay > 0 {
    //         key_emulation_delay -= 1;
    //     }

    //     #[cfg(feature = "tcpstream_keyboard")]
    //     match rx.try_recv() {
    //         Ok(key) => {
    //             if key_emulation_delay > 0 {
    //                 // It's not possible to process same keys which were entered shortly after each other
    //                 for frame in 0..key_emulation_delay {
    //                     debug!("Keys received too fast. Running extra emulation frame: {}", frame);
    //                     emulator.emulate_frames(MAX_FRAME_DURATION).map_err(|err| error!("{:?}", err))
    //                     .map_err(|err| error!("{:?}", err))
    //                         .ok();
    //                 }
    //                 emulator.screen_buffer()
    //                 .blit(&mut display, color_conv)
    //                 .map_err(|err| error!("{:?}", err))
    //                     .ok();
    //             }

    //             if key == last_key {
    //                 // Same key requires bigger delay
    //                 key_emulation_delay = 6;
    //             } else {
    //                 key_emulation_delay = 4;
    //             }

    //             last_key = key;

    //             info!("Key: {} - {}", key, true);
    //             let mapped_key_down_option = ascii_code_to_zxkey(key, true)
    //             .or_else(|| ascii_code_to_modifier(key, true));

    //             let mapped_key_down = match mapped_key_down_option {
    //                 Some(x) => { x },
    //                 _ => { Event::NoEvent }
    //             };

    //             let mapped_key_up_option = ascii_code_to_zxkey(key, false)
    //             .or_else(|| ascii_code_to_modifier(key, false));

    //             let mapped_key_up = match mapped_key_up_option {
    //                 Some(x) => { x },
    //                 _ => { Event::NoEvent }
    //             };

    //             debug!("-> key down");
    //             match mapped_key_down {
    //                 Event::ZXKey(k,p) => {
    //                     debug!("-> ZXKey");
    //                     emulator.send_key(k, p);
    //                 },
    //                 Event::ZXKeyWithModifier(k, k2, p) => {
    //                     debug!("-> ZXKeyWithModifier");
    //                     emulator.send_key(k, p);
    //                     emulator.send_key(k2, p);
    //                 }
    //                 _ => {
    //                     debug!("Unknown key.");
    //                 }
    //             }

    //             debug!("-> emulating frame");
    //             match emulator.emulate_frames(MAX_FRAME_DURATION) {
    //                 Ok(_) => {
    //                     emulator.screen_buffer()
    //                         .blit(&mut display, color_conv)
    //                         .map_err(|err| error!("{:?}", err))
    //                         .ok();
    //                 }
    //                 _ => {
    //                   error!("Emulation of frame failed");
    //                 }
    //             }

    //             debug!("-> key up");
    //             match mapped_key_up {
    //                 Event::ZXKey(k,p) => {
    //                     emulator.send_key(k, p);
    //                 },
    //                 Event::ZXKeyWithModifier(k, k2, p) => {
    //                     emulator.send_key(k, p);
    //                     emulator.send_key(k2, p);
    //                 }
    //                 _ => {}
    //             }

    //         },
    //         _ => {
    //         }
    //     }
    }
}
