#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
#[allow(unused_imports)]
use panic_halt as _;
use rust_x_arduino::infrared::IrReceiver;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    ufmt::uwriteln!(&mut serial, "Start\r").unwrap_infallible();
    
    let mut ir = IrReceiver::new(&dp.EXINT, pins.d2, dp.TC2);

    ufmt::uwriteln!(&mut serial, "Dog\r").unwrap_infallible();

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    unsafe { avr_device::interrupt::enable() };

    loop {
        if let Some(cmd) = ir.get_cmd() {
            ufmt::uwriteln!(
                &mut serial,
                "Cmd: Adress: {}, Command: {}, repeat: {}\r",
                cmd.addr,
                cmd.cmd,
                cmd.repeat
            )
            .unwrap_infallible();
        } else {
            // No command received
            // ufmt::uwrite!(&mut serial, "\\").unwrap_infallible();
        }

        arduino_hal::delay_ms(100);
    }
}
