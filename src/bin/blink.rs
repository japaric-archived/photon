//! Blink the blue LED (D7)

#![deny(warnings)]

#![no_std]

extern crate particle_hal as hal;
extern crate photon;

use hal::gpio::{self, Enum_PinMode, pin_t};
use hal::delay;

const D7: pin_t = 7;

#[no_mangle]
pub unsafe extern "C" fn setup() {
    gpio::HAL_Pin_Mode(D7, Enum_PinMode::OUTPUT);
}

#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loopy() {
    gpio::HAL_GPIO_Write(D7, 0);
    delay::HAL_Delay_Milliseconds(250);
    gpio::HAL_GPIO_Write(D7, 1);
    delay::HAL_Delay_Milliseconds(250);
}

// required by rust executables, but unused by our application
fn main() {}
