use avr_device::interrupt::Mutex;
use core::cell;

// Code in this file is based on the Arduino core library written in C++
// https://github.com/arduino/ArduinoCore-avr/blob/c8c514c9a19602542bc32c7033f48fecbbda4401/cores/arduino/wiring.c#L45C6-L45C7

const CLOCK_CYCLES_PER_MICROSECOND: u32 = 16;
const MICROS_PER_TIMER_OVERFLOW: u32 = (64 * 256) / CLOCK_CYCLES_PER_MICROSECOND;
const MILLIS_INC: u32 = MICROS_PER_TIMER_OVERFLOW / 1000;
const FRACT_INC: u32 = (MICROS_PER_TIMER_OVERFLOW % 1000) >> 3;
const FRACT_MAX: u32 = 1000 >> 3;

static TIMER0_OVERFLOW_COUNT: Mutex<cell::Cell<u32>> = Mutex::new(cell::Cell::new(0));

static TIMER0_MILLIS: Mutex<cell::Cell<u32>> = Mutex::new(cell::Cell::new(0));

static mut TIMER0_FRACT: u32 = 0;

#[avr_device::interrupt(atmega328p)]
fn TIMER0_OVF() {
    let mut m = avr_device::interrupt::free(|cs| TIMER0_OVERFLOW_COUNT.borrow(cs).get());
    let mut f = unsafe { TIMER0_FRACT };

    m += MILLIS_INC;
    f += FRACT_INC;

    // The clock the way is set now overflows every 1.024 ms which is not exactly accurate, 
    // so we need to adjust the clock every 125th overflow
    if f >= FRACT_MAX {
        f -= FRACT_MAX;
        m += 1;
    }
    unsafe {
        TIMER0_FRACT = f;
    }
    avr_device::interrupt::free(|cs| {
        TIMER0_MILLIS.borrow(cs).set(m);
        TIMER0_OVERFLOW_COUNT.borrow(cs).update(|x| x + 1);
    })
}

pub struct Timer {
    pub(crate) register: arduino_hal::pac::TC0,
}

impl Timer {
    pub fn new(tc0: arduino_hal::pac::TC0) -> Self {
        // We turn on timer 0 overflow interrupt
        tc0.timsk0.write(|w| w.toie0().set_bit());
        // Set timer 0 to normal mode
        tc0.tccr0a.write(|w| w.wgm0().normal_top());
        // Set timer 0 prescaler to 64 (counter increments every 64 ticks)
        tc0.tccr0b.write(|w| w.cs0().prescale_64());
        Timer { register: tc0 }
    }

    pub fn init(&self) {
        
    }

    pub fn millis(&self) -> u32 {
        avr_device::interrupt::free(|cs| TIMER0_MILLIS.borrow(cs).get())
    }

    pub fn micros(&self) -> u32 {
        let mut overflows = 0;
        let mut timer = 0;
        avr_device::interrupt::free(|cs| {
            overflows = TIMER0_OVERFLOW_COUNT.borrow(cs).get();
            timer = self.register.tcnt0.read().bits();

            // Check if the timer overflow interrupt flag is set and the timer value is less than 255
            // if it is, then we must assume that there happened one more overflow than we read
            if self.register.tifr0.read().tov0().bit() && timer < 255u8 {
                overflows += 1;
            }
        });

        // Each overflow happens every 256 ticks, so this gives us the number of ticks from the start
        ((overflows << 8) + timer as u32) *
            // One timer tick is 1 000 000 us / (16 000 000 hz (cpu freq) / 64 (prescaler)) = 
            // 64 / (1 000 000 us / 16 00 000 hz) = 4 us which is our timer's resolution
            (64 / CLOCK_CYCLES_PER_MICROSECOND)
    }
}
