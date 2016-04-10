#![feature(lang_items)]
#![no_std]

mod ffi;
mod lang_items;

pub enum Pin {
    D7
}

impl Pin {
    fn i16(&self) -> i16 {
        use self::Pin::*;

        match *self {
            D7 => 7,
        }
    }
}

pub enum PinMode {
    Output,
}

impl PinMode {
    fn u8(&self) -> u8 {
        use self::PinMode::*;

        match *self {
            Output => 1,
        }
    }
}

pub enum PinState {
    Low,
    High,
}

impl PinState {
    fn u8(&self) -> u8 {
        use self::PinState::*;

        match *self {
            Low => 0,
            High => 1,
        }
    }
}

pub fn pin_mode(pin: Pin, mode: PinMode) {
    unsafe {
        ffi::HAL_Pin_Mode(pin.i16(), mode.u8())
    }
}

pub fn digital_write(pin: Pin, value: PinState) {
    unsafe {
        ffi::HAL_GPIO_Write(pin.i16(), value.u8())
    }
}

pub fn delay(ms: u32) {
    unsafe {
        ffi::HAL_Delay_Milliseconds(ms)
    }
}
