#![no_std]

pub enum BoardType {
    ESP32C6DevKitC1,
    ESP32S3Box,
    M5StackCoreS3,
    M5StackFire
}

#[macro_export]
macro_rules! lcd_gpios {
    (BoardType::ESP32C6DevKitC1, $io:ident) => {
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
    (BoardType::ESP32S3Box, $io:ident) => {
        (
            $io.pins.gpio7,     // lcd_sclk
            $io.pins.gpio6,     // lcd_mosi
            $io.pins.gpio5,    // lcd_cs
            $io.pins.gpio2,     // lcd_miso
            $io.pins.gpio4.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio45.into_push_pull_output(),     // lcd_backlight
            $io.pins.gpio48.into_push_pull_output()      // lcd_reset
        )
    };
    (BoardType::M5StackCoreS3, $io:ident) => {
        (
            $io.pins.gpio36,    // lcd_sclk
            $io.pins.gpio37,    // lcd_mosi
            $io.pins.gpio3,     // lcd_cs
            $io.pins.gpio6,     // lcd_miso
            $io.pins.gpio35.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio0.into_push_pull_output(),    // lcd_backlight
            $io.pins.gpio15.into_push_pull_output()     // lcd_reset
        )
    };
    (BoardType::M5StackFire, $io:ident) => {
        (
            $io.pins.gpio18,    // lcd_sclk
            $io.pins.gpio23,    // lcd_mosi
            $io.pins.gpio5,     // lcd_cs
            $io.pins.gpio19,    // lcd_miso
            $io.pins.gpio26.into_push_pull_output(),    // lcd_dc
            $io.pins.gpio14.into_push_pull_output(),    // lcd_backlight
            $io.pins.gpio27.into_push_pull_output()     // lcd_reset
        )
    };
}

#[macro_export]
macro_rules! define_display_type {
    (BoardType::ESP32C6DevKitC1) => {
        mipidsi::Display<crate::spi_dma_displayinterface::SPIInterface<'static, GpioPin<Output<hal::gpio::PushPull>, 21>,
            GpioPin<Output<hal::gpio::PushPull>, 0>,
            hal::peripherals::SPI2, hal::gdma::Channel0, FullDuplexMode>,
            mipidsi::models::ILI9341Rgb565,
            GpioPin<Output<hal::gpio::PushPull>,
            3
        >>
    };
    (BoardType::ESP32S3Box) => {
        mipidsi::Display<crate::spi_dma_displayinterface::SPIInterface<'static, GpioPin<Output<hal::gpio::PushPull>, 4>,
            GpioPin<Output<hal::gpio::PushPull>, 0>,
            hal::peripherals::SPI2, hal::gdma::Channel0, FullDuplexMode>,
            mipidsi::models::ILI9342CRgb565,
            GpioPin<Output<hal::gpio::PushPull>,
            48
        >>
    };
    (BoardType::M5StackCoreS3) => {
        mipidsi::Display<crate::spi_dma_displayinterface::SPIInterface<'static, GpioPin<Output<esp32s3_hal::gpio::PushPull>, 35>,
            GpioPin<Output<esp32s3_hal::gpio::PushPull>, 0>,
            esp32s3_hal::peripherals::SPI2, esp32s3_hal::gdma::Channel0, FullDuplexMode>,
            mipidsi::models::ILI9342CRgb565,
            GpioPin<Output<esp32s3_hal::gpio::PushPull>,
            15
        >>
    };
}