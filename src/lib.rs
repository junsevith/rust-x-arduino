#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]

use arduino_hal::port::Pin;
use avr_hal_generic::port::mode::{Floating, Input};
use avr_hal_generic::port::PinOps;

pub mod timing;
pub mod servo;
pub mod screen;
pub mod movement;
pub mod echo;
pub mod infrared;


/// Type of some pin given by macro arduino_hal::pins!(dp) without any changes made to mode
pub type SomePin<T> = Pin<Input<Floating>, T>;