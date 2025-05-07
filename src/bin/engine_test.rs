#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(cell_update)]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use panic_halt as _;
use rust_x_arduino::interrupts::RotCounter;
use rust_x_arduino::movement::engine::Engine;
use rust_x_arduino::timing::millis::Timer;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let timer = Timer::new(dp.TC0);
    timer.init_pwm();

    let mut left_engine = Engine::new(
        timer.create_pwm_pin(pins.d6.into_output()),
        pins.d7.into_output(),
        pins.d4.into_output(),
    );

    let mut right_engine = Engine::new(
        timer.create_pwm_pin(pins.d5.into_output()),
        pins.d12.into_output(),
        pins.d8.into_output(),
    );

    let counter = RotCounter::new(
        dp.EXINT,
        pins.a0.into_floating_input(),
        pins.a1.into_floating_input(),
    );

    unsafe { avr_device::interrupt::enable() };

    left_engine.set_speed(255);
    right_engine.set_speed(255);
    left_engine.forward();
    right_engine.forward();


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
