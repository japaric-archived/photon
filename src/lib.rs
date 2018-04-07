//! Glue between the Particle firmware and the Rust language

#![deny(warnings)]
#![feature(lang_items)]
#![no_std]

extern crate photon_core;

mod lang_items;

pub use photon_core::{App, Cloud, Resource};

#[macro_export]
macro_rules! app {
    (setup: $setup:ident,loop: $loop:ident,) => {
        fn main() {}

        pub mod __impl {
            fn validate_signature(_: fn(app: $crate::App)) {}

            #[no_mangle]
            pub extern "C" fn setup() {
                let app = unsafe { ::core::ptr::read(0x0 as *const _) };

                validate_signature(::$setup);

                ::$setup(app);
            }

            #[export_name = "loop"]
            pub extern "C" fn loop_() {
                let app = unsafe { ::core::ptr::read(0x0 as *const _) };

                validate_signature(::$loop);

                ::$loop(app);
            }
        }
    };
}
