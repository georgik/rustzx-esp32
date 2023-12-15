
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

const REGION_WIDTH: usize = 8;
const REGION_HEIGHT: usize = 8;
const MAX_DIRTY_REGIONS: usize = 80;

pub(crate) struct DirtyRegion {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub(crate) struct EmbeddedGraphicsFrameBuffer {
    buffer: Vec<Rgb565>,
    buffer_width: usize,
    pub dirty_regions: Vec<DirtyRegion>,
    dirty_count: usize,
}

use crate::color_conv;
impl EmbeddedGraphicsFrameBuffer {
    pub fn get_pixel_iter(&self) -> impl Iterator<Item = Rgb565> + '_ {
        self.buffer.iter().copied()
    }


    fn mark_dirty(&mut self, x: usize, y: usize) {
        let region_start_x = x - (x % REGION_WIDTH);
        let region_start_y = y - (y % REGION_HEIGHT);
    
        for region in &mut self.dirty_regions {
            // Check if new pixel falls within or directly to the right of existing region
            if region.y == region_start_y && region.x <= region_start_x && x < region.x + region.width + REGION_WIDTH {
                // Extend region width in 8-pixel increments to cover the new pixel
                while x >= region.x + region.width {
                    region.width += REGION_WIDTH;
                }
                return;
            }
        // Vertical extension: Check if new pixel falls within or directly below existing region
        if region.x == region_start_x && region.y <= region_start_y && y < region.y + region.height + REGION_HEIGHT {
            // Extend region height in 8-pixel increments to cover the new pixel
            while y >= region.y + region.height {
                region.height += REGION_HEIGHT;
            }
            return;
        }
        }
    
        // Add a new dirty region if not adjacent to an existing region
        if self.dirty_regions.len() < MAX_DIRTY_REGIONS {
            self.dirty_regions.push(DirtyRegion { 
                x: region_start_x, 
                y: region_start_y, 
                width: REGION_WIDTH, 
                height: REGION_HEIGHT
            });
            self.dirty_count += 1;
        }
    }
        


    pub fn get_region_pixel_iter(&self, region: &DirtyRegion) -> impl Iterator<Item = Rgb565> + '_ {
        let start_x = region.x;
        let end_x = start_x + region.width;
        let start_y = region.y;
        let end_y = start_y + region.height;

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
                    dirty_regions: Vec::new(),
                    dirty_count: 0,
                }
            }
            FrameBufferSource::Border => Self {
                buffer: vec![Rgb565::WHITE; 1],
                buffer_width: 1,
                dirty_regions: Vec::new(),
                dirty_count: 0,
            },
        }
    }

    fn set_color(&mut self, x: usize, y: usize, zx_color: ZXColor, zx_brightness: ZXBrightness) {
        let index = y * self.buffer_width + x;
        let new_color = color_conv(&zx_color, zx_brightness);
        if self.buffer[index] != new_color {
            self.buffer[index] = new_color;
            self.mark_dirty(x, y);  // Mark the region as dirty
        }
    }

    fn set_colors(&mut self, x: usize, y: usize, colors: [ZXColor; 8], brightness: ZXBrightness) {
        for (i, &color) in colors.iter().enumerate() {
            self.set_color(x + i, y, color, brightness);
        }
    }

    // Reset dirty regions
    fn reset_dirty_regions(&mut self) {
        self.dirty_regions.clear();
        self.dirty_count = 0;
    }
    
}
