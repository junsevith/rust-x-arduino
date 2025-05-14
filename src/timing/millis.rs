use core::cell;

const CLOCK_CYCLES_PER_MICROSECOND: u32 = 16;
const MICROS_PER_TIMER_OVERFLOW: u32 = (64 * 256) / CLOCK_CYCLES_PER_MICROSECOND;
const MILLIS_INC: u32 = MICROS_PER_TIMER_OVERFLOW / 1000;
const FRACT_INC: u32 = (MICROS_PER_TIMER_OVERFLOW % 1000) >> 3;
const FRACT_MAX: u32 = 1000 >> 3;

static TIMER0_OVERFLOW_COUNT: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

static TIMER0_MILLIS: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

static mut TIMER0_FRACT: u32 = 0;

#[avr_device::interrupt(atmega328p)]
fn TIMER0_OVF() {
    let mut m = avr_device::interrupt::free(|cs| {
        TIMER0_OVERFLOW_COUNT.borrow(cs).get()
    });
    let mut f = unsafe { TIMER0_FRACT };

    m += MILLIS_INC;
    f += FRACT_INC;
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
        Timer {
            register: tc0,
        }
    }
    
    pub fn init(&self) {
        // We turn on timer 0 overflow interrupt
        self.register.timsk0.write(|w| w.toie0().set_bit());
        // Set timer 0 to normal mode
        self.register.tccr0a.write(|w| w.wgm0().normal_top());
        // Set timer 0 prescaler to 64 (counter increments every 64 ticks)
        self.register.tccr0b.write(|w| w.cs0().prescale_64());
    }

    pub fn millis(&self) -> u32 {
        avr_device::interrupt::free(|cs| TIMER0_MILLIS.borrow(cs).get())
    }

    pub fn micros(&self) -> u32 {
        let mut m = 0;
        let mut t = 0;
        avr_device::interrupt::free(|cs| {
            m = TIMER0_OVERFLOW_COUNT.borrow(cs).get();
            t = self.register.tcnt0.read().bits();
            if self.register.tifr0.read().tov0().bit() && t < 255u8 {
                // Timer 0 overflow interrupt flag is set
                m += 1;
            }
        });

        ((m << 8) + t as u32) * (64 / CLOCK_CYCLES_PER_MICROSECOND)
    }
}

pub fn millis() -> u32 {
    avr_device::interrupt::free(|cs| TIMER0_MILLIS.borrow(cs).get())
}