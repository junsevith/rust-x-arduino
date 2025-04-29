use arduino_hal::port::mode::{Floating, Input};
use arduino_hal::port::{Pin, A0, A1};
use avr_device::interrupt::Mutex;
use core::cell::Cell;
use core::mem::MaybeUninit;

type LeftPin = Pin<Input<Floating>, A0>;
type RightPin = Pin<Input<Floating>, A1>;

static mut COUNTER_STATE: MaybeUninit<RotCounterState> = MaybeUninit::uninit();
static LEFT_COUNTER: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));
static RIGHT_COUNTER: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));

#[avr_device::interrupt(atmega328p)]
fn PCINT1() {
    if unsafe {(*COUNTER_STATE.as_mut_ptr()).pin_left.is_high()} {
        avr_device::interrupt::free(|cs| {
            LEFT_COUNTER.borrow(cs).update(|x| x + 1);
        });
    }
    if unsafe {(*COUNTER_STATE.as_mut_ptr()).pin_right.is_high()} {
        avr_device::interrupt::free(|cs| {
            RIGHT_COUNTER.borrow(cs).update(|x| x + 1);
        });
    }
}
pub struct RotCounter {
    register: arduino_hal::pac::EXINT,
}

struct RotCounterState {
    pin_left: LeftPin,
    pin_right: RightPin,
}

impl RotCounterState {
    pub fn new(pin_left: LeftPin, pin_right: RightPin) -> Self {
        RotCounterState {
            pin_left,
            pin_right,
        }
    }

    pub fn get_state(&self) -> (bool, bool) {
        (self.pin_left.is_high(), self.pin_right.is_high())
    }
}

impl RotCounter {
    pub fn new(register: arduino_hal::pac::EXINT, pin_left: LeftPin, pin_right: RightPin) -> Self {
        register.pcicr.write(|w| w.pcie().bits(2u8));
        register.pcmsk1.write(|w| w.pcint().bits(3u8));

        unsafe {
            COUNTER_STATE = MaybeUninit::new(RotCounterState::new(pin_left, pin_right));
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }

        RotCounter { register }
    }

    pub fn left_count(&self) -> u8 {
        avr_device::interrupt::free(|cs| {
            LEFT_COUNTER.borrow(cs).get()
        })
    }

    pub fn right_count(&self) -> u8 {
        avr_device::interrupt::free(|cs| {
            RIGHT_COUNTER.borrow(cs).get()
        })
    }
}
