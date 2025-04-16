#![no_std]
#![no_main]

use ag_lcd::{Blink, Cursor, LcdDisplay, Lines};
use panic_halt as _;
use port_expander::dev::pcf8574::Pcf8574;


#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let delay = arduino_hal::Delay::new();

    let sda = pins.a4.into_pull_up_input();
    let scl = pins.a5.into_pull_up_input();
    let mut led = pins.d13.into_output();

    let i2c_bus = arduino_hal::i2c::I2c::new(peripherals.TWI, sda, scl, 400000);
    let mut i2c_expander = Pcf8574::new(i2c_bus, true, true, true);

    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new_pcf8574(&mut i2c_expander, delay)
        .with_cols(16)
        .with_lines(Lines::TwoLines)
        // .with_blink(Blink::On)
        .with_cursor(Cursor::On)
        .with_reliable_init(10000)
        .build();

    let slowa = [
        "baba",
        "mama",
        "tata",
        "kota",
        "pies",
        "kot",
        "piesek",
        "kotka",
    ];

    for slowo in slowa {
        lcd.clear();
        lcd.print(slowo);
        arduino_hal::delay_ms(500);
        led.toggle();
    }

    loop {
        arduino_hal::delay_ms(500);
        led.toggle();
    }
}
