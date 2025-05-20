use crate::timing::millis::Timer;
use arduino_hal::port::mode::Output;
use arduino_hal::port::{Pin, PinOps, D5, D6};
use crate::SomePin;

impl Timer {
    pub fn init_pwm(&self) {
        // We turn on timer 0 overflow interrupt
        self.register.timsk0.write(|w| w.toie0().set_bit());
        // Set timer 0 to normal mode
        self.register.tccr0a.write(|w| w.wgm0().pwm_fast());
        // Set timer 0 prescaler to 64 (counter increments every 64 ticks)
        self.register.tccr0b.write(|w| w.cs0().prescale_64());
    }

    pub fn create_pwm_pin<PIN: PinOps + PwmReady>(&self, pin: SomePin<PIN>) -> PwmPin<PIN> {
        PwmPin {
            _pin: pin.into_output(),
            register: &self.register,
        }
    }
}

pub struct PwmPin<'a, PIN: PinOps + PwmReady> {
    _pin: Pin<Output, PIN>,
    register: &'a arduino_hal::pac::TC0,
}

pub trait PwmReady {}
impl PwmReady for D5 {}
impl PwmReady for D6 {}

pub trait PwmPinOps {
    fn enable(&mut self);
    fn disable(&mut self);
    fn set_duty(&mut self, duty: u8);
    fn set_duty_percent(&mut self, duty: u8) {
        self.set_duty((duty * 255) / 100);
    }
    fn get_duty(&mut self) -> u8;
}

impl PwmPinOps for PwmPin<'_, D5> {
    fn enable(&mut self) {
        self.register.tccr0a.modify(|_, w| w.com0a().match_clear());
    }

    fn disable(&mut self) {
        self.register.tccr0a.modify(|_, w| w.com0a().disconnected());
    }

    fn set_duty(&mut self, duty: u8) {
        self.register.ocr0a.write(|x| x.bits(duty));
    }

    fn get_duty(&mut self) -> u8 {
        self.register.ocr0a.read().bits()
    }
}

impl PwmPinOps for PwmPin<'_, D6> {
    fn enable(&mut self) {
        self.register.tccr0a.modify(|_, w| w.com0b().match_clear());       
    }

    fn disable(&mut self) {
        self.register.tccr0a.modify(|_, w| w.com0b().disconnected());
    }

    fn set_duty(&mut self, duty: u8) {
        self.register.ocr0b.write(|x| x.bits(duty));
    }

    fn get_duty(&mut self) -> u8 {
        self.register.ocr0b.read().bits()
    }
}
