pub mod engine;
pub mod counters;

use crate::movement::engine::Engine;
use crate::timing::pwm::PwmPinOps;
use arduino_hal::port::PinOps;
use crate::movement::counters::RotCounter;

pub struct Movement<LPWM, LF, LB, RPWM, RF, RB>
where
    LPWM: PwmPinOps,
    LF: PinOps,
    LB: PinOps,
    RPWM: PwmPinOps,
    RF: PinOps,
    RB: PinOps,
{
    left: Engine<LPWM, LF, LB>,
    right: Engine<RPWM, RF, RB>,
    pub rot_counter: RotCounter,
}

impl<LPWM, LF, LB, RPWM, RF, RB> Movement<LPWM, LF, LB, RPWM, RF, RB>
where
    LPWM: PwmPinOps,
    LF: PinOps,
    LB: PinOps,
    RPWM: PwmPinOps,
    RF: PinOps,
    RB: PinOps,
{
    pub fn new(
        left: Engine<LPWM, LF, LB>,
        right: Engine<RPWM, RF, RB>,
        rot_counter: RotCounter,
    ) -> Self {
        Self {
            left,
            right,
            rot_counter,
        }
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.left.set_speed(speed);
        self.right.set_speed(speed);
    }

    pub fn forward(&mut self) {
        self.left.forward();
        self.right.forward();
    }

    pub fn backward(&mut self) {
        self.left.backward();
        self.right.backward();
    }

    pub fn stop(&mut self) {
        self.left.stop();
        self.right.stop();
    }

    pub fn left(&mut self) {
        self.left.stop();
        self.right.forward();
    }

    pub fn right(&mut self) {
        self.left.forward();
        self.right.stop();
    }
}
