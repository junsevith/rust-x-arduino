#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
#[allow(unused_imports)]
use panic_halt as _;
use rust_x_arduino::movement::counters::RotCounter;
use rust_x_arduino::timing::millis::Timer;

//Przetestować to, bo źle działa na wokwi
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let timer = Timer::new(dp.TC0);
    timer.init_pwm();

    let counter = RotCounter::new(
        &dp.EXINT,
        pins.a0.into_floating_input(),
        pins.a1.into_floating_input(),
    );

    unsafe { avr_device::interrupt::enable() };

    loop {
        for i in 0..1000 {
            ufmt::uwriteln!(
                &mut serial,
                "time: {}, left: {}, right {}",
                i,
                counter.left_count(),
                counter.right_count()
            )
            .unwrap_infallible();
            arduino_hal::delay_ms(1000);
        }
    }
}
