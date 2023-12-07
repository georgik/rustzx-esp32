use embedded_hal::can::Frame;
use log::*;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

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
const LCD_PIXELS: usize = LCD_H_RES*1;
// use rustzx_utils::io::FileAsset;
// use rustzx_utils::stopwatch::InstantStopwatch;
use crate::stopwatch::InstantStopwatch;
use crate::io::FileAsset;
use crate::spritebuf::SpriteBuf;

use embedded_graphics_framebuf::FrameBuf;
use embedded_graphics_framebuf::backends::{DMACapableFrameBufferBackend, FrameBufferBackend};
use embedded_graphics::pixelcolor::Rgb565;

use display_interface::WriteOnlyDataCommand;
use embedded_hal::digital::v2::OutputPin;

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
    type DebugInterface = StubDebugInterface; // TODO
}

pub(crate) struct Esp32HostContext;

impl HostContext<Esp32Host> for Esp32HostContext
{
    fn frame_buffer_context(&self) -> <<Esp32Host as Host>::FrameBuffer as FrameBuffer>::Context {
        ()
    }
}

pub(crate) struct EmbeddedGraphicsFrameBuffer {
    buffer: [ZXColor; LCD_PIXELS],
    buffer_width: usize,
    // changed: RefCell<Vec<bool>>,
}

use crate::color_conv;
impl EmbeddedGraphicsFrameBuffer {
    pub fn get_pixel_iter(&self) -> impl Iterator<Item = Rgb565> + '_ {
        self.buffer.into_iter().map(|zh_color| color_conv(zh_color, ZXBrightness::Normal))
    }
}

// impl EmbeddedGraphicsFrameBuffer {
    // pub(crate) fn blit<D: DrawTarget>(
    //     &self,
    //     display: &mut D,
    //     color_conv: fn(ZXColor, ZXBrightness) -> D::Color,
    // ) -> Result<(), D::Error> {

        // let mut changed = self.changed.borrow_mut();

        // let mut y = 0_usize;
        // while y < changed.len() {
        //     let mut yoff = y;
        //     while yoff < changed.len() && changed[yoff] {
        //         changed[yoff] = false;
        //         yoff += 1;

        //         break; // TODO: Seems there is a bug with multiple rows
        //     }

        //     if y < yoff {
        //         display.fill_contiguous(
        //             &Rectangle::new(
        //                 Point::new(0, y as i32),
        //                 Size::new(self.buffer_width as u32, (yoff - y) as u32),
        //             ),
        //             self.buffer[y * self.buffer_width..yoff * self.buffer_width]
        //                 .iter()
        //                 .map(|zh_color| color_conv(*zh_color, ZXBrightness::Normal)),
        //         )?;

        //         y = yoff;
        //     } else {
        //         y += 1;
        //     }
        // }

//         Ok(())
//     }
// }

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
                    buffer: [ZXColor::Red; LCD_PIXELS],
                    buffer_width: LCD_H_RES as usize,
                    // changed: RefCell::new(vec![true; height]),
                }
            }
            // FrameBufferSource::Border => todo!("Border frame buffer not implemented"),
            FrameBufferSource::Border => Self {
                buffer: [ZXColor::White; LCD_PIXELS],
                buffer_width: LCD_H_RES as usize,
                // changed: RefCell::new(Vec::new()),
            },
        }
    }

    fn set_color(
        &mut self,
        x: usize,
        y: usize,
        color: ZXColor,
        _brightness: ZXBrightness, /*TODO*/
    ) {
        if self.buffer_width > 0 {
            // let pixel = &mut self.buffer[y * self.buffer_width + x];
            // if *pixel as u8 != color as u8 {
                // *pixel = color;
                // self.changed.borrow_mut()[y] = true;
            // }
        }
    }
}

pub struct StubDebugInterface;
use rustzx_core::host::DebugInterface;

impl DebugInterface for StubDebugInterface {
    fn check_pc_breakpoint(&mut self, _addr: u16) -> bool {
        // In a stub implementation, you can simply return false
        // to indicate that no breakpoints are set.
        false
    }
}
