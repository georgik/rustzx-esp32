
extern crate alloc;
use alloc::{vec, vec::Vec};
use embedded_graphics::pixelcolor::RgbColor;
use log::*;

use rustzx_core::host::FrameBuffer;
use rustzx_core::host::FrameBufferSource;
use rustzx_core::host::Host;
use rustzx_core::host::HostContext;
use rustzx_core::host::StubIoExtender;
use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
// use spooky_embedded::embedded_display::LCD_H_RES;
// use spooky_embedded::embedded_display::LCD_PIXELS;
const LCD_H_RES: usize = 256;
const LCD_PIXELS: usize = LCD_H_RES*192;
// use rustzx_utils::io::FileAsset;
// use rustzx_utils::stopwatch::InstantStopwatch;
use crate::stopwatch::InstantStopwatch;
use crate::io::FileAsset;
use embedded_graphics::pixelcolor::Rgb565;


pub(crate) struct Esp32Host
{
}

impl Host for Esp32Host
{
    type Context = Esp32HostContext;
    type EmulationStopwatch = InstantStopwatch;
    type FrameBuffer = EmbeddedGraphicsFrameBuffer;
    type TapeAsset = FileAsset; // TODO
    type IoExtender = StubIoExtender;
    // type DebugInterface = StubDebugInterface; // TODO
}

pub(crate) struct Esp32HostContext;

impl HostContext<Esp32Host> for Esp32HostContext
{
    fn frame_buffer_context(&self) -> <<Esp32Host as Host>::FrameBuffer as FrameBuffer>::Context {
        ()
    }
}

pub(crate) struct EmbeddedGraphicsFrameBuffer {
    buffer: Vec<Rgb565>,
    buffer_width: usize,
    // changed: RefCell<Vec<bool>>,
}

use crate::color_conv;
impl EmbeddedGraphicsFrameBuffer {
    pub fn get_pixel_iter(&self) -> impl Iterator<Item = Rgb565> + '_ {
        self.buffer.iter().copied()
    }
}


impl FrameBuffer for EmbeddedGraphicsFrameBuffer {
    type Context = ();

    fn new(
        width: usize,
        height: usize,
        source: FrameBufferSource,
        _context: Self::Context,
    ) -> Self {
        info!("Allocation");
        match source {
            FrameBufferSource::Screen => {
                info!("Allocating frame buffer width={}, height={}", width, height);

                Self {
                    buffer: vec![Rgb565::RED; LCD_PIXELS],
                    buffer_width: LCD_H_RES as usize,
                }
            }
            FrameBufferSource::Border => Self {
                buffer: vec![Rgb565::WHITE; 1],
                buffer_width: 1,
            },
        }
    }

    fn set_color(
        &mut self,
        x: usize,
        y: usize,
        zx_color: ZXColor,
        zx_brightness: ZXBrightness,
    ) {
        self.buffer[y * self.buffer_width + x] = color_conv(&zx_color, zx_brightness);
        // if *pixel != color {
        //     *pixel = color;
        // }
    }
}

pub struct StubDebugInterface;
// use rustzx_core::host::DebugInterface;

// impl DebugInterface for StubDebugInterface {
//     fn check_pc_breakpoint(&mut self, _addr: u16) -> bool {
//         // In a stub implementation, you can simply return false
//         // to indicate that no breakpoints are set.
//         false
//     }
// }
