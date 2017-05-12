set -ex

main() {
    xargo build --target $TARGET
}

main
