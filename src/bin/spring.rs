#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(cell_update)]

#[allow(unused_imports)]
use panic_halt as _;
use rust_x_arduino::echo::Echo;
use rust_x_arduino::movement::counters::RotCounter;
use rust_x_arduino::movement::engine::Engine;
use rust_x_arduino::movement::Movement;
use rust_x_arduino::servo::Servo;
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
        &dp.EXINT,
        pins.a0,
        pins.a1,
    );

    let mut servo = Servo::new(dp.TC1, pins.d9.into_output());
    let mut movement = Movement::new(left_engine, right_engine, counter);
    let mut echo = Echo::new(pins.d10.into_output(), pins.d11.into_floating_input());

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    unsafe { avr_device::interrupt::enable() };

    servo.set_angle(90);

    const GOAL: i32 = 50;

    loop {
        let dist = echo.distance(&timer) as i32 - GOAL;

        let speed: u8 = match u8::try_from(dist.abs()) {
            Ok(speed) => speed.saturating_mul(32),
            Err(_) => {u8::MAX}
        };

        ufmt::uwriteln!(&mut serial, "speed {}:", speed).unwrap();

        if dist < -5 {
            movement.backward();
        } else if dist > 5 {
            movement.forward();
        } else {
            movement.stop();
        }

        movement.set_speed(speed);
    }
}
