use arduino_hal::delay_us;
use arduino_hal::port::mode::{Floating, Input, Output};
use arduino_hal::port::{Pin, PinOps};
use crate::timing::millis::Timer;

pub struct Echo<T: PinOps, E: PinOps> {
    trigger: Pin<Output, T>,
    echo: Pin<Input<Floating>, E>,
}

impl<T: PinOps, E: PinOps> Echo<T, E> {
    pub fn new(trigger: Pin<Output, T>, echo: Pin<Input<Floating>, E>) -> Self {
        Self { trigger, echo }
    }

    pub fn distance(&mut self, timer: &Timer) -> u32 {
        let echo_start = timer.millis();

        self.trigger.set_high();
        delay_us(10);
        self.trigger.set_low();

        while self.echo.is_low() {
            if timer.millis() - echo_start > 200 {
                return u32::MAX;
            }
        }

        let start = timer.micros();

        while self.echo.is_high() {}
        
        let duration = timer.micros() - start;
        
        duration / 58
    }
}