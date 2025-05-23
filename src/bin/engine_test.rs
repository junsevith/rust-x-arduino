#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::delay_ms;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
#[allow(unused_imports)]
use panic_halt as _;
use rust_x_arduino::movement::counters::RotCounter;
use rust_x_arduino::movement::engine::Engine;
use rust_x_arduino::timing::millis::Timer;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let timer = Timer::new(dp.TC0);
    timer.init_pwm();

    let mut left_engine = Engine::new(timer.create_pwm_pin(pins.d6), pins.d7, pins.d4);

    let mut right_engine = Engine::new(timer.create_pwm_pin(pins.d5), pins.d12, pins.d8);

    let counter = RotCounter::new(&dp.EXINT, pins.a0, pins.a1);

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    unsafe { avr_device::interrupt::enable() };

    left_engine.set_speed(255);
    right_engine.set_speed(255);
    left_engine.forward();
    delay_ms(1000);
    right_engine.forward();

    loop {
        for i in 0..10 {
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
        left_engine.stop();
        right_engine.stop();
    }
}
