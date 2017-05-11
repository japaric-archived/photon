set -ex

main() {
    dpkg --add-architecture i386

    apt-get update

    # NOTE(libarchive-zip-perl) crc32
    # NOTE(vim-common) xxd command
    apt-get install -y --no-install-recommends \
            bzip2 \
            ca-certificates \
            curl \
            libarchive-zip-perl \
            make \
            sudo \
            vim-common

    cd /usr/local
    curl -L https://launchpad.net/gcc-arm-embedded/4.9/4.9-2015-q1-update/+download/gcc-arm-none-eabi-4_9-2015q1-20150306-linux.tar.bz2 | \
        tar --strip-components 1 -xj

    apt-get install -y --no-install-recommends \
            libc6:i386

    echo 'ALL ALL=(ALL) NOPASSWD: ALL' | (EDITOR="tee -a" visudo)

    rm $0
}

main
