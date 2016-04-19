#!/bin/bash

set -ex

main() {
    local tag=2016-04-10

    docker run -v $(pwd):/mnt -w /mnt japaric/photon:$tag bash -ex -c '
        rustup default nightly
        cargo install xargo
        xargo build --release --verbose
        arm-none-eabi-size $(find target/photon/release -maxdepth 1 -type f -executable)
    '
}

main
