#!/bin/bash

set -ex

main() {
    local tag=2016-04-26

    # The particle user has id = 1000, but this may not match the travis user id. To workaround this
    # issue, make everything world write-able.
    chmod -R a+w .

    # NOTE(mv) the .cargo/config sets photon as the default target. Momentarily disable it while
    # installing xargo.
    docker run -v $(pwd):/mnt -w /mnt japaric/photon:$tag bash -ex -c '
        rustup default nightly
        xargo build --release --verbose
        arm-none-eabi-size $(find target/photon/release -maxdepth 1 -type f -executable)
    '
}

main
