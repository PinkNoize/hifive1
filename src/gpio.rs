///
/// Returns single pin for given gpio object mapped accordingly
/// 
/// *mappings*
/// 
///     - spi_<what> -- SPI pins where <what> is one of (sck, mosi, miso, ss0, ss2, ss3)
///     - i2c_<what> -- I2C pins where <what> is one of (sda, scl)
///     - serial_<what> -- Serial pins where <what> is one of (tx, rx)
///     - dig# -- Digital/physical pins on the board where # is from range <0..19>
/// 
/// *example*
/// 
/// `pin!(gpio.spi_mosi) -> gpio.pin3`
/// 
#[macro_export]
macro_rules! pin {
    // empty
    ($gpio:ident, none) => { () };
    // spi
    ($gpio:ident, spi_sck) => { $gpio.pin5 };
    ($gpio:ident, spi_mosi) => { $gpio.pin3 };
    ($gpio:ident, spi_miso) => { $gpio.pin4 };
    ($gpio:ident, spi_ss0) => { $gpio.pin2 };
    // spi_ss1 is not documented
    ($gpio:ident, spi_ss2) => { $gpio.pin9 };
    ($gpio:ident, spi_cs3) => { $gpio.pin10 };
    // i2c
    ($gpio:ident, i2c_sda) => { $gpio.pin12 };
    ($gpio:ident, i2c_scl) => { $gpio.pin13 };
    // serial
    ($gpio:ident, serial_tx) => { $gpio.pin17 };
    ($gpio:ident, serial_rx) => { $gpio.pin16 };
    // digital/physical
    ($gpio:ident, dig0) => { $gpio.pin16 };
    ($gpio:ident, dig1) => { $gpio.pin17 };
    ($gpio:ident, dig2) => { $gpio.pin18 };
    ($gpio:ident, dig3) => { $gpio.pin19 };
    ($gpio:ident, dig4) => { $gpio.pin20 };
    ($gpio:ident, dig5) => { $gpio.pin21 };
    ($gpio:ident, dig6) => { $gpio.pin22 };
    ($gpio:ident, dig7) => { $gpio.pin23 };
    ($gpio:ident, dig8) => { $gpio.pin0 };
    ($gpio:ident, dig9) => { $gpio.pin1 };
    ($gpio:ident, dig10) => { $gpio.pin2 };
    ($gpio:ident, dig11) => { $gpio.pin3 };
    ($gpio:ident, dig12) => { $gpio.pin4 };
    ($gpio:ident, dig13) => { $gpio.pin5 };
    ($gpio:ident, dig14) => { $gpio.pin8 }; // tested
    ($gpio:ident, dig15) => { $gpio.pin9 };
    ($gpio:ident, dig16) => { $gpio.pin10 };
    ($gpio:ident, dig17) => { $gpio.pin11 };
    ($gpio:ident, dig18) => { $gpio.pin12 };
    ($gpio:ident, dig19) => { $gpio.pin13 };
}

///
/// Returns tuple of pins for given gpio object mapped accordingly
/// 
/// *mappings*
/// 
///     - none -- Returns () for empty pin if needed in tuple
///     - spi_<what> -- SPI pins where <what> is one of (sck, mosi, miso, ss0, ss2, ss3)
///     - i2c_<what> -- I2C pins where <what> is one of (sda, scl)
///     - serial_<what> -- Serial pins where <what> is one of (tx, rx)
///     - dig# -- Digital/physical pins on the board where # is from range <0..19>
/// 
/// *example*
/// 
/// `pins!(gpio, (spi_mosi, spi_miso, spi_sck, spi_ss0)) -> (gpio.pin3, gpio.pin4, gpio.pin5, gpio.pin2)`
/// 
#[macro_export]
macro_rules! pins {
    ( $gpio:ident, ($($name:ident),+) ) => {
        ($($crate::pin_name!($gpio, $name)),+)
    }
}