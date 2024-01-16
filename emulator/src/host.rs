
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
const LCD_H_RES: usize = 256;
const LCD_PIXELS: usize = LCD_H_RES*192;
use crate::stopwatch::InstantStopwatch;
use crate::io::FileAsset;
use embedded_graphics::pixelcolor::Rgb565;

use graphics::color_conv;

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
    pub bounding_box_top_left: Option<(usize, usize)>,
    pub bounding_box_bottom_right: Option<(usize, usize)>,
}

impl EmbeddedGraphicsFrameBuffer {
    // pub fn get_pixel_iter(&self) -> impl Iterator<Item = Rgb565> + '_ {
    //     self.buffer.iter().copied()
    // }

    fn mark_dirty(&mut self, x: usize, y: usize) {
        let (min_x, min_y) = self.bounding_box_top_left.unwrap_or((x, y));
        let (max_x, max_y) = self.bounding_box_bottom_right.unwrap_or((x, y));

        self.bounding_box_top_left = Some((min_x.min(x), min_y.min(y)));
        self.bounding_box_bottom_right = Some((max_x.max(x), max_y.max(y)));
    }

    pub fn get_region_pixel_iter(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> impl Iterator<Item = Rgb565> + '_ {
        let start_x = top_left.0;
        let end_x = bottom_right.0 + 1; // Include the pixel at bottom_right coordinates
        let start_y = top_left.1;
        let end_y = bottom_right.1 + 1; // Include the pixel at bottom_right coordinates

        (start_y..end_y).flat_map(move |y| {
            (start_x..end_x).map(move |x| {
                self.buffer[y * self.buffer_width + x]
            })
        })
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
                    bounding_box_bottom_right: None,
                    bounding_box_top_left: None,
                }
            }
            FrameBufferSource::Border => Self {
                buffer: vec![Rgb565::WHITE; 1],
                buffer_width: 1,
                bounding_box_bottom_right: None,
                bounding_box_top_left: None,
            },
        }
    }


    fn set_color(&mut self, x: usize, y: usize, zx_color: ZXColor, zx_brightness: ZXBrightness) {
        let index = y * self.buffer_width + x;
        let new_color = color_conv(&zx_color, zx_brightness);
        if self.buffer[index] != new_color {
            self.buffer[index] = new_color;
            self.mark_dirty(x, y);  // Update the bounding box
        }
    }

    fn set_colors(&mut self, x: usize, y: usize, colors: [ZXColor; 8], brightness: ZXBrightness) {
        for (i, &color) in colors.iter().enumerate() {
            self.set_color(x + i, y, color, brightness);
        }
    }

    fn reset_bounding_box(&mut self) {
        self.bounding_box_bottom_right = None;
        self.bounding_box_top_left = None;
    }

}
