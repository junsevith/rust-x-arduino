#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(cell_update)]
#![allow(static_mut_refs)]
pub mod timing;
pub mod servo;
pub mod screen_format;
pub mod interrupts;
pub mod movement;
pub mod echo;