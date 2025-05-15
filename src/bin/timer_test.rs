#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

#[allow(unused_imports)]
use panic_halt as _;
use rust_x_arduino::timing::millis::Timer;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    let timer = Timer::new(dp.TC0);
    timer.init();

    unsafe { avr_device::interrupt::enable() };

    loop {
        for i in (10..1000).step_by(10) {
            let start = timer.micros();
            arduino_hal::delay_us(i);
            let end = timer.micros();

            let duration = end - start;

            let error = duration - i;

            let percent = (error * 100) / i;
            ufmt::uwriteln!(&mut serial, "{} : {} : {} : {}", duration, i, error, percent).unwrap();

            arduino_hal::delay_ms(100);
        }
    }
}
