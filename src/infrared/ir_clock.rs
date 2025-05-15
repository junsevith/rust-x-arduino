use avr_device::interrupt::Mutex;
use core::cell::Cell;

pub static CLOCK: IrClock = IrClock::new();
#[avr_device::interrupt(atmega328p)]
fn TIMER2_COMPA() {
    avr_device::interrupt::free(|cs| CLOCK.counter.borrow(cs).update(|val| val + 1))
}

pub struct IrClock {
    counter: Mutex<Cell<u32>>,
}

impl Default for IrClock {
    fn default() -> Self {
        Self::new()
    }
}

impl IrClock {
    pub(crate) const FREQ: u32 = 20_000;
    const TOP: u8 = 99;

    pub const fn new() -> Self {
        Self {
            counter: Mutex::new(Cell::new(0)),
        }
    }

    pub fn start(&self, tc2: arduino_hal::pac::TC2) {
        // We set up the timer to count at 20kHz
        tc2.tccr2a.write(|w| w.wgm2().ctc());
        tc2.ocr2a.write(|w| w.bits(Self::TOP));
        tc2.tccr2b.write(|w| w.cs2().prescale_8());

        // Enable interrupt
        tc2.timsk2.write(|w| w.ocie2a().set_bit());
    }

    pub fn now(&self) -> u32 {
        avr_device::interrupt::free(|cs| self.counter.borrow(cs).get())
    }
}
