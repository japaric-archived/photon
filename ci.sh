#!/bin/bash

main() {
    local tag=2016-04-10

    docker run japaric/photon:$tag sh -c "
        rustup default nightly
        cargo install --git https://github.com/japaric/xargo
        git clone --depth 1 https://github.com/japaric/photon
        cd photon
        xargo build --release
        arm-none-eabi-size $(find target/photon/release -type f -executable)
        arm-none-eabi-objdump -CD $(find target/photon/release -type f -executable)
    "
}

main
