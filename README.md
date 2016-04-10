[![Travis](https://travis-ci.org/japaric/photon.svg?branch=master)](https://travis-ci.org/japaric/photon)

# `photon`

> Cargo project template to get you started with the [particle] [photon].

[particle]: https://www.particle.io/
[photon]: https://store.particle.io/collections/photon

## Dependencies

- All the dependencies required to build [spark/firmware].
- [xargo], a transparent cargo wrapper that handles sysroots (cross compiled standard crates) for
you.

[spark/firmware]: https://github.com/spark/firmware/blob/develop/docs/dependencies.md
[xargo]: https://github.com/japaric/xargo

## Docker

If you don't want to deal with dependencies you can use a Docker image that comes with everything
pre-installed.

```
$ docker run -it japaric/photon:2016-04-10
# or use a newer tag. See https://hub.docker.com/r/japaric/photon/tags/
```

## Example usage

```
# Write your app in the `src/bin` or `src/examples` folder
$ cat src/bin/blink.rs
//! Blink the blue LED (D7)

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

# Build your app -- you get an ELF file back
# This may take a while (a few minutes) the first time because we have to build spark/firmware.
# and a sysroot! Subsequent calls should be way faster!
$ xargo build --release

# Inspect the ELF file
$ file target/photon/release/blink
target/photon/release/blink: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped

$ arm-none-eabi-size target/photon/release/blink
      text    data     bss     dec     hex filename
      6004    1068     188    7260    1c5c target/photon/release/blink

# Turn the ELF file into a binary for deploy
# binarify.sh also computes the CRC and write it into the binary
$ ./binarify.sh target/photon/release/blink

# Ship the app to the Photon
$ particle flash $device_name blinky.bin

# Enjoy!
```

## Suggestions

- You probably want to modify the behavior of the `lang_items::panic_fmt` function to control the
behavior of your program when it panics.

## Caveats

- You can only build your apps inside this Cargo project. I'd like to break this repository into a
`photon-sys` crate that you can link into another Cargo project to build Photon apps. I'm
envisioning a process like this:

```
$ cargo new --bin my_app

$ cd my_app

$ cargo add photon-sys

# write app
$ edit src/main.rs

# TODO need to some command here to fetch the photon.json file

$ xargo build --target photon

# TODO a cargo subcommand to directly flash the app into the photon
```

- There are only a few HAL functions available in this repository. I don't plan to add more here. I
think a HAL library with FFI and rustic bindings should be developed in a different crate/repository.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
