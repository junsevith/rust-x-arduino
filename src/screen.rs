use ag_lcd::{LcdDisplay, Lines};
use arduino_hal::hal::port::{PC4, PC5};
use arduino_hal::hal::Atmega;
use arduino_hal::port::mode::{Floating, Input};
use arduino_hal::{DefaultClock, Delay, I2c};
use avr_device::atmega328p::TWI;
use core::cell::RefCell;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use port_expander::dev::pcf8574::Driver;
use port_expander::mode::QuasiBidirectional;
use port_expander::Pcf8574;

pub struct Screen<T: OutputPin, D: DelayNs> {
    pub lcd: LcdDisplay<T, D>,
}

impl Screen<port_expander::Pin<'_, QuasiBidirectional, RefCell<Driver<avr_hal_generic::i2c::I2c<Atmega, TWI, avr_hal_generic::port::Pin<Input, PC4>, avr_hal_generic::port::Pin<Input, PC5>, DefaultClock>>>>, Delay> {
    
    pub fn i2c(p:TWI, sda: arduino_hal::port::Pin<Input<Floating>,PC4>, scl: arduino_hal::port::Pin<Input<Floating>, PC5>) -> Pcf8574<RefCell<Driver<I2c>>> {
        let i2c_bus = I2c::new(p, sda.into_pull_up_input(), scl.into_pull_up_input(), 400000);
        Pcf8574::new(i2c_bus, true, true, true)
    }
    pub fn new(i2c: &mut Pcf8574<RefCell<Driver<I2c>>>) -> Screen<port_expander::Pin<QuasiBidirectional, RefCell<Driver<avr_hal_generic::i2c::I2c<Atmega, TWI, avr_hal_generic::port::Pin<Input, PC4>, avr_hal_generic::port::Pin<Input, PC5>, DefaultClock>>>>, Delay>
    {
        let delay = Delay::new();
        let lcd = LcdDisplay::new_pcf8574(i2c, delay)
            .with_cols(16)
            .with_lines(Lines::TwoLines)
            .build();
        
        Screen {
            lcd,
        }
    }
}

impl<T, D> ufmt::uWrite for Screen<T, D>
where
    D: DelayNs + Sized,
    T: OutputPin + Sized,
{
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.lcd.print(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.lcd.write(c as u8);
        Ok(())
    }
}