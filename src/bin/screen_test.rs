#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(cell_update)]

use rust_x_arduino::timing::pwm::PwmPinOps;
use ag_lcd::{LcdDisplay, Lines};
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use port_expander::dev::pcf8574::Pcf8574;
use rust_x_arduino::screen_format::Screen;
use rust_x_arduino::servo::Servo;
use rust_x_arduino::timing::millis::Timer;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    let delay = arduino_hal::Delay::new();

    let sda = pins.a4.into_pull_up_input();
    let scl = pins.a5.into_pull_up_input();
    
    let i2c_bus = arduino_hal::i2c::I2c::new(dp.TWI, sda, scl, 400000);
    let mut i2c_expander = Pcf8574::new(i2c_bus, true, true, true);

    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new_pcf8574(&mut i2c_expander, delay)
        .with_cols(16)
        .with_lines(Lines::TwoLines)
        .build();
    
    let mut screen = Screen {
        lcd,
    };

    unsafe { avr_device::interrupt::enable() };

    let words = [
        "Hello",
        "World",
        "This",
        "is",
        "a",
        "test",
    ];

    let custom_char = [
        0b01110,
        0b10001,
        0b10001,
        0b01110,
        0b00100,
        0b00111,
        0b00100,
        0b00111,
    ];

    screen.lcd.set_character(0, custom_char);

    for word in words {
        screen.lcd.clear();
        ufmt::uwrite!(&mut screen, "{}", word).unwrap();
        arduino_hal::delay_ms(500);
    }

    screen.lcd.clear();
    screen.lcd.write(0);

    loop {

    }
}
