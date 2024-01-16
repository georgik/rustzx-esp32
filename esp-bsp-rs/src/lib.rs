#![no_std]

#[macro_export]
macro_rules! lcd_gpios {
    ("ESP32-C6-DevKitC-1", $io:ident) => {
        (
            $io.pins.gpio6,     // lcd_sclk
            $io.pins.gpio7,     // lcd_mosi
            $io.pins.gpio20,    // lcd_cs
            $io.pins.gpio0,     // lcd_miso
            $io.pins.gpio21.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio4.into_push_pull_output(),     // lcd_backlight
            $io.pins.gpio3.into_push_pull_output()      // lcd_reset
        )
    };
}

#[macro_export]
macro_rules! define_display_type {
    ("ESP32-C6-DevKitC-1") => {
        mipidsi::Display<crate::spi_dma_displayinterface::SPIInterface<'static, GpioPin<Output<hal::gpio::PushPull>, 21>,
            GpioPin<Output<hal::gpio::PushPull>, 0>,
            hal::peripherals::SPI2, hal::gdma::Channel0, FullDuplexMode>,
            mipidsi::models::ILI9341Rgb565,
            GpioPin<Output<hal::gpio::PushPull>,
            3
        >>
    };
}