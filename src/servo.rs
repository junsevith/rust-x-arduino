use arduino_hal::port::mode::Output;
use arduino_hal::port::D9;

pub struct Servo {
    timer: arduino_hal::pac::TC1,
    pin: arduino_hal::port::Pin<Output, D9>,
}

const MAX_LEFT: u32 = 115;
const MAX_RIGHT: u32 = 620;

const INTERVAL: u32 = MAX_RIGHT - MAX_LEFT;

impl Servo {
    pub fn new(timer: arduino_hal::pac::TC1, pin: arduino_hal::port::Pin<Output, D9>) -> Self {
        let mut new = Servo { timer, pin };
        new.init();
        new
    }

    fn init(&mut self) {
        // We set the top value for timer1 to 4999 so the pwm frequency is 50Hz
        self.timer.icr1.write(|w| w.bits(4999));
        // we set waveform generation mode to fast pwm (0b1100) with icr1 top
        // we set com1a (pin 9) to clear on match
        // we set prescaler to 64
        self.timer
            .tccr1a
            .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
        self.timer
            .tccr1b
            .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
    }
    
    pub fn set_angle(&mut self, angle: u8) {
        // 100 counts => 0.4ms
        // 700 counts => 2.8ms
        let duty = ((angle as u32 * INTERVAL) / 180) + MAX_LEFT;
        self.timer.ocr1a.write(|w| w.bits(duty as u16));
    }
    
    pub fn set_duty(&mut self, duty: u16) {
        self.timer.ocr1a.write(|w| w.bits(duty));
    }
}
