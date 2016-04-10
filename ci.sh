#!/bin/bash

set -ex

main() {
    local tag=2016-04-10

    docker run japaric/photon:$tag bash -ex -c '
        rustup default nightly
        cargo install --git https://github.com/japaric/xargo
        git clone --depth 1 https://github.com/japaric/photon
        cd photon
        xargo build --release --verbose
        arm-none-eabi-size $(find target/photon/release -maxdepth 1 -type f -executable)
    '
}

main
