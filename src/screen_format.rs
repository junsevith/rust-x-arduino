use ag_lcd::LcdDisplay;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

pub struct Screen<T: OutputPin, D: DelayNs> {
    pub lcd: LcdDisplay<T, D>,
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