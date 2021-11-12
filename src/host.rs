use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use log::*;

use embedded_graphics::prelude::*;

use rustzx_core::host::FrameBuffer;
use rustzx_core::host::FrameBufferSource;
use rustzx_core::host::Host;
use rustzx_core::host::HostContext;
use rustzx_core::host::StubIoExtender;
use rustzx_core::zx::video::colors::ZXBrightness;
use rustzx_core::zx::video::colors::ZXColor;
use rustzx_utils::io::FileAsset;
use rustzx_utils::stopwatch::InstantStopwatch;

pub(crate) struct Esp32Host<D>(PhantomData<D>);

impl<D> Host for Esp32Host<D>
where
    D: DrawTarget,
    D::Error: core::fmt::Debug,
{
    type Context = Esp32HostContext<D>;
    type EmulationStopwatch = InstantStopwatch;
    type FrameBuffer = EmbeddedGraphicsFrameBuffer<D>;
    type TapeAsset = FileAsset; // TODO
    type IoExtender = StubIoExtender;
}

pub(crate) struct Esp32HostContext<D: DrawTarget> {
    display: Rc<RefCell<D>>,
    color_conv: fn(ZXColor, ZXBrightness) -> D::Color,
}

impl<D: DrawTarget> Esp32HostContext<D> {
    pub(crate) fn new(display: D, color_conv: fn(ZXColor, ZXBrightness) -> D::Color) -> Self {
        Self {
            display: Rc::new(RefCell::new(display)),
            color_conv,
        }
    }
}

impl<D: DrawTarget> Clone for Esp32HostContext<D> {
    fn clone(&self) -> Self {
        Self {
            display: self.display.clone(),
            color_conv: self.color_conv,
        }
    }
}

impl<D> HostContext<Esp32Host<D>> for Esp32HostContext<D>
where
    D: DrawTarget,
    D::Error: core::fmt::Debug,
{
    fn frame_buffer_context(
        &self,
    ) -> <<Esp32Host<D> as Host>::FrameBuffer as FrameBuffer>::Context {
        self.clone()
    }
}

pub(crate) struct EmbeddedGraphicsFrameBuffer<D: DrawTarget> {
    display: Option<Rc<RefCell<D>>>,
    color_conv: fn(ZXColor, ZXBrightness) -> D::Color,
    buffer: Vec<ZXColor>,
    buffer_width: usize,
}

impl<D> FrameBuffer for EmbeddedGraphicsFrameBuffer<D>
where
    D: DrawTarget,
    D::Error: core::fmt::Debug,
{
    type Context = Esp32HostContext<D>;

    fn new(width: usize, height: usize, source: FrameBufferSource, context: Self::Context) -> Self {
        match source {
            FrameBufferSource::Screen => {
                let mut buffer: Vec<ZXColor> = Vec::new(); // TODO: Optimize storage
                for y in 0..height {
                    for x in 0..width {
                        buffer.push(ZXColor::Red);
                    }
                }

                info!("Allocated frame buffer width={}, height={}", width, height);

                Self {
                    display: Some(context.display),
                    buffer,
                    buffer_width: width,
                    color_conv: context.color_conv,
                }
            }
            FrameBufferSource::Border => Self {
                display: None,
                buffer: Vec::new(),
                buffer_width: 0,
                color_conv: context.color_conv,
            },
        }
    }

    fn set_color(&mut self, x: usize, y: usize, color: ZXColor, brightness: ZXBrightness) {
        if let Some(display) = self.display.as_mut() {
            let pixel = &mut self.buffer[y * self.buffer_width + x];
            if *pixel as u8 != color as u8 {
                *pixel = color;

                let mut display = display.borrow_mut();

                Pixel(
                    Point::new(x as i32, y as i32),
                    (self.color_conv)(color, brightness),
                )
                .draw(&mut *display)
                .unwrap();
            }
        }
    }
}
