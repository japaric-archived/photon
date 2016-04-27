//! Turn D7 (blue LED) ON

#![deny(warnings)]

#![no_std]

extern crate particle_hal as hal;
extern crate photon;

use hal::gpio::{self, Enum_PinMode, pin_t};

const D7: pin_t = 7;

#[no_mangle]
pub unsafe extern "C" fn setup() {
    gpio::HAL_Pin_Mode(D7, Enum_PinMode::OUTPUT);
    gpio::HAL_GPIO_Write(D7, 1);
}

#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loopy() {
}

// required by rust executables, but unused by our application
fn main() {}
