# `photon`

> Binary blobs for building photon apps

Firmware version: v0.6.2

# Documentation

For user level documentation, check
the [photon-quickstart](https://github.com/japaric/photon-quickstart) template.

# Generating binary blobs

```
$ cd docker && docker build -t photon-builder . && cd ..

$ git clone https://github.com/spark/firmware

$ cd firmware

$ git checkout v0.6.2

$ ../docker/run.sh
```

# License

The Rust code in repository is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The binary blobs in this repository were generated from the [spark/firmware]
repository and as such they are licensed according to [their terms].

[spark/firmware]: https://github.com/spark/firmware/tree/v0.6.2
[their terms]: https://github.com/spark/firmware/tree/v0.6.2#license

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
