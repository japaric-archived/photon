#!/bin/bash

set -ex

input=$1
name=$(basename $input)

cleanup() {
    rm $name.bin.*
}

main() {
    arm-none-eabi-objcopy -O binary $input $name.bin.pre_crc

    head -c $(($(stat -c %s $name.bin.pre_crc) - 38)) $name.bin.pre_crc > $name.bin.no_crc

    tail -c 38 $name.bin.pre_crc > $name.bin.crc_block

    [ "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20280078563412" = $(xxd -p -c 500 $name.bin.crc_block) ] || (cleanup && exit 1)

    shasum -a 256 $name.bin.no_crc | \
        cut -c 1-65 | \
        xxd -r -p | \
        dd bs=1 of=$name.bin.pre_crc seek=$(($(stat -c %s $name.bin.pre_crc) - 38)) conv=notrunc

    head -c $(($(stat -c %s $name.bin.pre_crc) - 4)) $name.bin.pre_crc > $name.bin.no_crc

    crc32 $name.bin.no_crc | \
        cut -c 1-10 | \
        xxd -r -p | \
        dd bs=1 of=$name.bin.pre_crc seek=$(($(stat -c %s $name.bin.pre_crc) - 4)) conv=notrunc

    mv $name.bin.pre_crc $name.bin

    cleanup
}

main
