use crate::timing::pwm::PwmPinOps;
use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps};

pub struct Engine<PWM: PwmPinOps, F: PinOps, B: PinOps> {
    pwm: PWM,
    forward: Pin<Output, F>,
    backward: Pin<Output, B>,
}

impl<PWM, F, B> Engine<PWM, F, B>
where
    PWM: PwmPinOps,
    F: PinOps,
    B: PinOps,
{
    pub fn new(mut pwm: PWM, forward: Pin<Output, F>, backward: Pin<Output, B>) -> Self {
        pwm.enable();
        Self {
            pwm,
            forward,
            backward,
        }
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.pwm.set_duty(speed);
    }

    pub fn forward(&mut self) {
        self.forward.set_high();
        self.backward.set_low();
    }

    pub fn backward(&mut self) {
        self.forward.set_low();
        self.backward.set_high();
    }
    
    pub fn stop(&mut self) {
        self.forward.set_low();
        self.backward.set_low();
    }
}
