#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]
pub mod timing;
pub mod servo;
pub mod screen;
pub mod movement;
pub mod echo;
pub mod infrared;