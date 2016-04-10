//! Turn D7 (blue LED) ON

#![deny(warnings)]

#![no_std]

extern crate photon;

use photon::{Pin, PinMode, PinState};

#[no_mangle]
pub fn setup() {
    photon::pin_mode(Pin::D7, PinMode::Output);
    photon::digital_write(Pin::D7, PinState::High);
}

#[no_mangle]
#[export_name = "loop"]
pub fn loopy() {
}

// required by rust executables, but unused by our application
fn main() {}
