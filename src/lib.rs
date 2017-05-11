//! Glue between the Particle firmware and the Rust language

#![deny(warnings)]
#![feature(lang_items)]
#![no_std]

mod lang_items;

#[macro_export]
macro_rules! app {
    (setup: $setup:ident, loop: $loop:ident,) => {
        fn main() {}

        pub mod __impl {
            fn validate_signature(_: fn()) {}

            #[no_mangle]
            pub extern "C" fn setup() {
                validate_signature(::$setup);

                ::$setup()
            }

            #[export_name = "loop"]
            pub extern "C" fn loop_() {
                validate_signature(::$loop);

                ::$loop()
            }
        }
    }
}
