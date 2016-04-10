//! Do nothing! D7 (blue LED) should be OFF

#![deny(warnings)]

#![no_std]

extern crate photon;

#[no_mangle]
pub fn setup() {}

#[no_mangle]
#[export_name = "loop"]
pub fn loopy() {}

// required by rust executables, but unused by our application
fn main() {}
