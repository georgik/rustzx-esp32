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

    emulate_zx(peripherals, display::create!(peripherals)?, display::color_conv)
}






use core::cell::RefCell;

use core::mem::MaybeUninit;
use esp32_hal::{
    gpio::{Gpio26, Gpio27},
    pac::Peripherals,
    prelude::*,
    RtcCntl, Timer, IO,
};
use esp_hal_common::{interrupt, pac, Cpu, Event, Floating, Input, Pin};
// use esp_println::println;
// use panic_halt as _;
use pc_keyboard::{layouts, HandleControl, ScancodeSet2};
// use riscv::interrupt::Mutex;
// use riscv_rt::entry;

static mut CLK: Mutex<RefCell<Option<Gpio26<Input<Floating>>>>> = Mutex::new(RefCell::new(None));
static mut DATA: Mutex<RefCell<Option<Gpio27<Input<Floating>>>>> = Mutex::new(RefCell::new(None));



static mut QUEUE: Option<SimpleQueue<u8, 5>> = None;

fn send_byte(byte: u8) {
    riscv::interrupt::free(|_| unsafe {
        if QUEUE.is_none() {
            QUEUE = Some(SimpleQueue::new());
        }
        match QUEUE {
            Some(ref mut queue) => {
                queue.enqueue(byte);
            }
            None => (),
        }
    });
}

fn get_byte() -> Option<u8> {
    riscv::interrupt::free(|_| unsafe {
        match QUEUE {
            Some(ref mut queue) => queue.dequeue(),
            None => None,
        }
    })
}

#[no_mangle]
pub fn interrupt3() {
    static mut BIT_COUNT: usize = 0;
    static mut CURRENT: u8 = 0;

    riscv::interrupt::free(|cs| unsafe {
        let mut clk = CLK.borrow(*cs).borrow_mut();
        let clk = clk.as_mut().unwrap();

        let mut data = DATA.borrow(*cs).borrow_mut();
        let data = data.as_mut().unwrap();

        let bit = if data.is_high().unwrap() { 1 } else { 0 };

        interrupt::clear(Cpu::ProCpu, interrupt::CpuInterrupt::Interrupt3);
        clk.clear_interrupt();

        if BIT_COUNT > 0 && BIT_COUNT < 9 {
            CURRENT = CURRENT.overflowing_shr(1).0;
            CURRENT |= bit << 7;
        }
        BIT_COUNT += 1;

        if BIT_COUNT == 11 {
            send_byte(CURRENT);

            BIT_COUNT = 0;
            CURRENT = 0;
        }
    });
}

pub struct SimpleQueue<T, const N: usize> {
    data: [Option<T>; N],
    read_index: usize,
    write_index: usize,
}

impl<T, const N: usize> SimpleQueue<T, N> {
    pub fn new() -> SimpleQueue<T, N> {
        let mut queue = unsafe {
            SimpleQueue {
                data: MaybeUninit::uninit().assume_init(),
                read_index: 0,
                write_index: 0,
            }
        };

        for i in 0..N {
            queue.data[i] = None;
        }

        queue
    }

    pub fn enqueue(&mut self, e: T) -> bool {
        self.data[self.write_index] = Some(e);

        self.write_index += 1;
        self.write_index %= N;

        if self.write_index == self.read_index {
            return false;
        }

        true
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.write_index == self.read_index {
            None
        } else {
            let result = self.data[self.read_index].take();
            self.read_index += 1;
            self.read_index %= N;
            result
        }
    }

    pub fn is_empty(&self) -> bool {
        self.read_index == self.write_index
    }

    pub fn is_full(&self) -> bool {
        let mut next_write = self.read_index + 1;
        next_write %= N;

        next_write == self.read_index
    }
}







use std::sync::mpsc::{channel, Sender, Receiver}; 

fn handle_client(mut stream: TcpStream, tx:Sender<u8>) {
    let mut data = [0 as u8; 256]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            for n in 0..size {
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


pub enum ZXEvent {
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
fn ascii_code_to_zxkey(ascii_code: u8, pressed: bool) -> Option<ZXEvent> {
    let zxkey_event = match ascii_code {
        // Control keys
        0x10 => Some(ZXKey::Enter),
        0x13 => Some(ZXKey::Enter),
        // Temporary Enter
        0x40 => Some(ZXKey::Enter),

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

    zxkey_event.map(|k| ZXEvent::ZXKey(k, pressed))
}


/// returns ZX Spectum key form scancode of None if not found
fn ascii_code_to_modifier(ascii_code: u8, pressed: bool) -> Option<ZXEvent> {
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

    zxkey_event.map(|(k, k2)| ZXEvent::ZXKeyWithModifier(k, k2, pressed))
}



fn emulate_zx<D>(peripherals: Peripherals, mut display: D, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> Result<()>
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






    // let peripherals = Peripherals::take().unwrap();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
    let mut timer0 = Timer::new(peripherals.TIMG0);
    let mut timer1 = Timer::new(peripherals.TIMG1);

    rtc_cntl.set_super_wdt_enable(false);
    rtc_cntl.set_wdt_enable(false);
    timer0.disable();
    timer1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut data_out = io.pins.gpio12.into_open_drain_output();
    let mut clk_out = io.pins.gpio14.into_open_drain_output();

    let data_in = io.pins.gpio27.into_floating_input();
    let mut clk_in = io.pins.gpio26.into_floating_input();
    clk_in.listen(Event::FallingEdge);

    data_out.set_low().unwrap();
    clk_out.set_low().unwrap();

    data_out.set_high().unwrap();
    clk_out.set_high().unwrap();

    riscv::interrupt::free(|_cs| unsafe {
        CLK.get_mut().replace(Some(clk_in));
        DATA.get_mut().replace(Some(data_in));
    });

    interrupt::enable(
        Cpu::ProCpu,
        pac::Interrupt::GPIO,
        interrupt::CpuInterrupt::Interrupt3,
    );
    interrupt::set_kind(
        Cpu::ProCpu,
        interrupt::CpuInterrupt::Interrupt3,
        interrupt::InterruptKind::Level,
    );
    interrupt::set_priority(
        Cpu::ProCpu,
        interrupt::CpuInterrupt::Interrupt3,
        interrupt::Priority::Priority1,
    );

    unsafe {
        riscv::interrupt::enable();
    }

    let mut kb = pc_keyboard::Keyboard::new(
        layouts::Us104Key,
        ScancodeSet2,
        HandleControl::MapLettersToUnicode,
    );
    loop {
        if let Some(byte) = get_byte() {
            match kb.add_byte(byte) {
                Ok(Some(event)) => {
                    println!("Event {:?}", event);
                }
                Ok(None) => (),
                Err(e) => {
                    println!("Error decoding: {:?}", e);
                }
            }
        }
    }






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



    loop {
        const MAX_FRAME_DURATION: Duration = Duration::from_millis(0);

        let duration = emulator
            .emulate_frames(MAX_FRAME_DURATION);

        // info!("Rendering 60 frames took {}ms", duration.as_millis().unwrap());

        // TODO: Screen should be constantly updated from within the emulation cycle, using multithreading
        emulator
            .screen_buffer()
            .blit(&mut display, color_conv);


        match rx.try_recv() {
            Ok(key) => {
  
                println!("Key: {} - {}", key, true);
                let mapped_key_down = ascii_code_to_zxkey(key, true)
                .or_else(|| ascii_code_to_modifier(key, true)).unwrap();

                let mapped_key_up = ascii_code_to_zxkey(key, false)
                .or_else(|| ascii_code_to_modifier(key, false)).unwrap();
                            
                match mapped_key_down {
                    ZXEvent::ZXKey(k,p) => {
                        emulator.send_key(k, p);        
                    },
                    ZXEvent::ZXKeyWithModifier(k, k2, p) => {
                        emulator.send_key(k, p);
                        emulator.send_key(k2, p);
                    }
                    _ => {}
                }

                emulator.emulate_frames(MAX_FRAME_DURATION);
                emulator.screen_buffer()
                    .blit(&mut display, color_conv);

                match mapped_key_up {
                    ZXEvent::ZXKey(k,p) => {
                        emulator.send_key(k, p);        
                    },
                    ZXEvent::ZXKeyWithModifier(k, k2, p) => {
                        emulator.send_key(k, p);
                        emulator.send_key(k2, p);
                    }
                    _ => {}
                }
            },
            _ => {
                emulator.emulate_frames(MAX_FRAME_DURATION);
                emulator.screen_buffer()
                .blit(&mut display, color_conv);        
            }
        }




        // Yield
        //thread::sleep(Duration::from_secs(0));
    }
}
