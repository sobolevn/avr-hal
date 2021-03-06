//! Implementation of the Rust Embedded-HAL SPI FullDuplex trait for AVR.
//!
//! The interface can be instantiated with the `new` method, and used directly
//! or passed into a driver.  Example usage:
//!
//! ```
//! // create SPI interface
//! let (mut spi, mut cs) = Spi::new(
//!     dp.SPI,// SPI peripheral
//!     pins.d52.into_output(&mut pins.ddr),// SCLK
//!     pins.d51.into_output(&mut pins.ddr),// MOSI output pin
//!     pins.d50.into_pull_up_input(&mut pins.ddr),// MISO input pin
//!     pins.d53.into_output(&mut pins.ddr),// CS pin
//!     Settings::default(),
//! );
//!
//! // Send a byte
//! let sent = 0b10101010;
//! spi.send(sent).unwrap();
//! let response = spi.read().unwrap();
//! ```
//! In the example above, all of the settings are left at the default.  You can
//! also instantiate a Settings object with the other options available.

extern crate avr_hal_generic as avr_hal;

pub use avr_hal_generic::spi::*;
use crate::port::portb;


avr_hal_generic::impl_spi! {
    pub struct Spi {
        peripheral: crate::pac::SPI,
        pins: {
            sclk: portb::PB1,
            mosi: portb::PB2,
            miso: portb::PB3,
            cs: portb::PB0,
        }
    }
}
