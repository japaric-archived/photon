//! Blink the blue LED (D7)

#![deny(warnings)]

#![no_std]

extern crate photon;

use photon::{Pin, PinMode, PinState};

#[no_mangle]
pub fn setup() {
    photon::pin_mode(Pin::D7, PinMode::Output)
}

#[no_mangle]
#[export_name = "loop"]
pub fn loopy() {
    photon::digital_write(Pin::D7, PinState::Low);
    photon::delay(250);
    photon::digital_write(Pin::D7, PinState::High);
    photon::delay(250);
}

// required by rust executables, but unused by our application
fn main() {}
