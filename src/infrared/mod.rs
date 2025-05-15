use crate::infrared::ir_clock::{IrClock, CLOCK};
use arduino_hal::port::{Pin, D2};
use avr_device::atmega328p::TC2;
use avr_device::interrupt::Mutex;
use avr_hal_generic::port::mode::{Floating, Input};
use core::cell::Cell;
use core::mem::MaybeUninit;
use infrared::{protocol::nec::NecCommand, protocol::*, Receiver};

pub mod ir_clock;

type IrPin = Pin<Input<Floating>, D2>;
type IrProto = Nec;
type IrCmd = NecCommand;
static mut RECEIVER: MaybeUninit<Receiver<IrProto, IrPin>> = MaybeUninit::uninit();

static CMD: Mutex<Cell<Option<IrCmd>>> = Mutex::new(Cell::new(None));
pub struct IrReceiver {}

impl IrReceiver {
    pub fn new(register: &arduino_hal::pac::EXINT, pin: IrPin, timer: TC2) -> Self {
        CLOCK.start(timer);
        
        pin.enable_pin_change_interrupt(register);
        

        unsafe {
            RECEIVER = MaybeUninit::new(Receiver::with_pin(IrClock::FREQ, pin));
        }

        Self {}
    }

    pub fn get_cmd(&mut self) -> Option<IrCmd> {
        avr_device::interrupt::free(|cs| CMD.borrow(cs).take())
    }
}

trait EnablePCINT {
    fn enable_pin_change_interrupt(&self, register: &arduino_hal::pac::EXINT);
}

impl EnablePCINT for IrPin {
    fn enable_pin_change_interrupt(&self, register: &arduino_hal::pac::EXINT) {
        // Enable group 2 (PORTD)
        register.pcicr.write(|w| w.pcie().bits(0b100));

        // Enable pin change interrupts on PCINT18 which is pin PD2 (= d2)
        register.pcmsk2.write(|w| w.bits(0b100));
    }
}

#[avr_device::interrupt(atmega328p)]
fn PCINT2() {
    let recv = unsafe { RECEIVER.assume_init_mut() };

    let now = CLOCK.now();

    match recv.event_instant(now) {
        Ok(Some(cmd)) => {
            avr_device::interrupt::free(|cs| {
                let cell = CMD.borrow(cs);
                cell.set(Some(cmd));
            });
        }
        Ok(None) => (),
        Err(_) => (),
    }
}
