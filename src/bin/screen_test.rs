#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

#[allow(unused_imports)]
use panic_halt as _;
use rust_x_arduino::screen::Screen;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut i2c = Screen::i2c(dp.TWI, pins.a4, pins.a5);
    let mut screen = Screen::new(&mut i2c);

    unsafe { avr_device::interrupt::enable() };

    let words = ["Hello", "World", "This", "is", "a", "test"];

    let custom_char = [
        0b01110, 0b10001, 0b10001, 0b01110, 0b00100, 0b00111, 0b00100, 0b00111,
    ];

    screen.lcd.set_character(0, custom_char);

    for word in words {
        screen.lcd.clear();
        ufmt::uwrite!(&mut screen, "{}", word).unwrap();
        arduino_hal::delay_ms(500);
    }

    screen.lcd.clear();
    screen.lcd.write(0);

    loop {}
}
