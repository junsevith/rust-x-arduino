#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use rust_x_arduino::servo::Servo;
#[allow(unused_imports)]
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let mut servo = Servo::new(dp.TC1, pins.d9);

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    unsafe { avr_device::interrupt::enable() };

    loop {
        for i in 0..180 {
            ufmt::uwriteln!(&mut serial, "{}", i).unwrap();

            servo.set_angle(i);

            arduino_hal::delay_ms(20);
        }
        arduino_hal::delay_ms(1000);
        servo.set_angle(0);
        arduino_hal::delay_ms(1000);
    }
}
