#![no_std]
#![no_main]

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
    gdma::Gdma,
    i2c,
    // interrupt,
    peripherals::{
        Peripherals,
        UART1
    },
    prelude::*,
    spi::{
        master::{prelude::*, Spi},
        SpiMode,
    },
    Delay,
    // Rng,
    IO,
    uart::{
        config::{Config, DataBits, Parity, StopBits},
        TxRxPins,
    },
    Uart
};

// use spooky_embedded::app::app_loop;

use spooky_embedded::{
    embedded_display::LCD_MEMORY_SIZE,
    // controllers::{accel::AccelMovementController, composites::accel_composite::AccelCompositeController}
};

use esp_backtrace as _;

// use icm42670::{Address, Icm42670};
use shared_bus::BusManagerSimple;

use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_core::{zx::machine::ZXMachine, EmulationMode, Emulator, RustzxSettings, host::Host};

use log::{info, error, debug};

use core::time::Duration;
use embedded_graphics::pixelcolor::Rgb565;

use display_interface::WriteOnlyDataCommand;
use mipidsi::models::Model;
use embedded_hal::digital::v2::OutputPin;

use core::mem::MaybeUninit;

use axp2101::{ I2CPowerManagementInterface, Axp2101 };
use aw9523::I2CGpioExpanderInterface;

use pc_keyboard::{layouts, HandleControl, ScancodeSet2};

mod host;
mod stopwatch;
mod io;
mod spritebuf;
mod pc_zxkey;
use pc_zxkey::{ pc_code_to_zxkey, pc_code_to_modifier };
mod zx_event;
use zx_event::Event;

use crate::io::FileAsset;

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

    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();

    esp_println::logger::init_logger_from_env();

    let mut delay = Delay::new(&clocks);

    info!("About to initialize the SPI LED driver");
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let lcd_sclk = io.pins.gpio36;
    let lcd_mosi = io.pins.gpio37;
    let lcd_cs = io.pins.gpio3;
    let lcd_miso = io.pins.gpio6;
    let lcd_dc = io.pins.gpio35.into_push_pull_output();
    let lcd_reset = io.pins.gpio15.into_push_pull_output();

    let serial_tx = io.pins.gpio17.into_push_pull_output();
    let serial_rx = io.pins.gpio18.into_floating_input();

    // I2C
    let sda = io.pins.gpio12;
    let scl = io.pins.gpio11;

    let dma = Gdma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    let mut descriptors = [0u32; 8 * 3];
    let mut rx_descriptors = [0u32; 8 * 3];

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

    let spi = Spi::new(
        peripherals.SPI3,
        lcd_sclk,
        lcd_mosi,
        lcd_miso,
        lcd_cs,
        60u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    )   .with_dma(dma_channel.configure(
        false,
        &mut descriptors,
        &mut rx_descriptors,
        DmaPriority::Priority0,
    ));

    delay.delay_ms(500u32);

    //https://github.com/m5stack/M5CoreS3/blob/main/src/utility/Config.h#L8
    let di = spi_dma_displayinterface::new_no_cs(LCD_MEMORY_SIZE, spi, lcd_dc);

    let mut display = mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(320, 240)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut delay, Some(lcd_reset))
        .unwrap();
    delay.delay_ms(500u32);
    info!("Initializing...");
    Text::new(
        "Initializing...",
        Point::new(80, 110),
        MonoTextStyle::new(&FONT_8X13, RgbColor::WHITE),
    )
    .draw(&mut display)
    .unwrap();

    info!("Initialized");

    // let mut rng = Rng::new(peripherals.RNG);
    // let mut seed_buffer = [0u8; 32];
    // rng.read(&mut seed_buffer).unwrap();

    // let accel_movement_controller = AccelMovementController::new(icm, 0.2);
    // let demo_movement_controller = spooky_core::demo_movement_controller::DemoMovementController::new(seed_buffer);
    // let movement_controller = AccelCompositeController::new(demo_movement_controller, accel_movement_controller);

    // app_loop( &mut display, seed_buffer, movement_controller);

    let config = Config {
        baudrate: 115200,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1,
    };

    let pins = TxRxPins::new_tx_rx(
        serial_tx,
        serial_rx,
    );

    let serial = Uart::new_with_config(peripherals.UART1, config, Some(pins), &clocks);

    let _ = app_loop(&mut display, color_conv, serial);
    loop {}

}


fn color_conv(color: &ZXColor, _brightness: ZXBrightness) -> Rgb565 {
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

fn handle_key_event<H: Host>(key: pc_keyboard::KeyCode, state: pc_keyboard::KeyState, emulator: &mut Emulator<H>) {
    let is_pressed = matches!(state, pc_keyboard::KeyState::Down);
    if let Some(mapped_key) = pc_code_to_zxkey(key, is_pressed).or_else(|| pc_code_to_modifier(key, is_pressed)) {
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

fn app_loop<DI, M, RST>(
    display: &mut mipidsi::Display<DI, M, RST>,
    _color_conv: fn(&ZXColor, ZXBrightness) -> Rgb565,
    mut serial: Uart<UART1>,
) //-> Result<(), core::fmt::Error>
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
        machine: ZXMachine::Sinclair128K,
        // machine: ZXMachine::Sinclair48K,
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



    info!("Loading tape");
    let tape_bytes = include_bytes!("../test.tap");
    let tape_asset = FileAsset::new(tape_bytes);
    let _ = emulator.load_tape(rustzx_core::host::Tape::Tap(tape_asset));

    info!("Entering emulator loop");
    let mut kb = pc_keyboard::Keyboard::new(
        ScancodeSet2::new(),
        layouts::Us104Key,
        HandleControl::MapLettersToUnicode,
    );

    loop {
        // info!("Emulating frame");
        let read_result = serial.read();
        match read_result {
            Ok(byte) => {
                match kb.add_byte(byte) {
                    Ok(Some(event)) => {
                        info!("Event {:?}", event);
                        handle_key_event(event.code, event.state, &mut emulator);
                    },
                    Ok(None) => {},
                    Err(_) => {},
                }
            }
            Err(_) => {},
        }

        // emulator.emulate_frames(MAX_FRAME_DURATION);
        match emulator.emulate_frames(MAX_FRAME_DURATION) {
            Ok(_) => {
                // info!("Emulation of frame succeeded");
                let pixel_iterator = emulator.screen_buffer().get_pixel_iter();
                // info!("Drawing frame");
                let _ = display.set_pixels(0, 0, 256 - 1, 192, pixel_iterator);
            }
            _ => {
              error!("Emulation of frame failed");
            }
        }
    }
}
