use std::cell::RefCell;
use std::time::Instant;

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
use rustzx_utils::io::FileAsset;
use rustzx_utils::stopwatch::InstantStopwatch;

pub(crate) struct Esp32Host;

impl Host for Esp32Host {
    type Context = Esp32HostContext;
    type EmulationStopwatch = InstantStopwatch;
    type FrameBuffer = EmbeddedGraphicsFrameBuffer;
    type TapeAsset = FileAsset; // TODO
    type IoExtender = StubIoExtender;
}

pub(crate) struct Esp32HostContext;

impl HostContext<Esp32Host> for Esp32HostContext {
    fn frame_buffer_context(&self) -> <<Esp32Host as Host>::FrameBuffer as FrameBuffer>::Context {
        ()
    }
}

pub(crate) struct EmbeddedGraphicsFrameBuffer {
    buffer: Vec<ZXColor>,
    buffer_width: usize,
    changed: RefCell<Vec<bool>>,
}

impl EmbeddedGraphicsFrameBuffer {
    pub(crate) fn blit<D: DrawTarget>(
        &self,
        display: &mut D,
        color_conv: fn(ZXColor, ZXBrightness) -> D::Color,
    ) -> Result<(), D::Error> {
        let start_time = Instant::now();

        let mut changed = self.changed.borrow_mut();

        let mut y = 0_usize;
        while y < changed.len() {
            let mut yoff = y;
            while yoff < changed.len() && changed[yoff] {
                changed[yoff] = false;
                yoff += 1;

                break; // TODO: Seems there is a bug with multiple rows
            }

            if y < yoff {
                display.fill_contiguous(
                    &Rectangle::new(
                        Point::new(0, y as i32),
                        Size::new(self.buffer_width as u32, (yoff - y) as u32),
                    ),
                    self.buffer[y * self.buffer_width..yoff * self.buffer_width]
                        .iter()
                        .map(|zh_color| color_conv(*zh_color, ZXBrightness::Normal)),
                )?;

                y = yoff;
            } else {
                y += 1;
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > 50 {
            info!("Screen blit took {}ms - slow", elapsed.as_millis());
        }

        Ok(())
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
        match source {
            FrameBufferSource::Screen => {
                info!("Allocating frame buffer width={}, height={}", width, height);

                Self {
                    buffer: vec![ZXColor::Red; width * height],
                    buffer_width: width,
                    changed: RefCell::new(vec![true; height]),
                }
            }
            FrameBufferSource::Border => Self {
                buffer: Vec::new(),
                buffer_width: 0,
                changed: RefCell::new(Vec::new()),
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
            let pixel = &mut self.buffer[y * self.buffer_width + x];
            if *pixel as u8 != color as u8 {
                *pixel = color;
                self.changed.borrow_mut()[y] = true;
            }
        }
    }
}
