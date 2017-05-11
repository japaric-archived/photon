set -ex

main() {
    docker run \
           --rm \
           -v `pwd`:/work \
           -w /work \
           -it photon-builder \
           sh -c "
groupadd -g $(id -g) $(id -g -n)
useradd -u $(id -u) -g $(id -g) $(id -u -n)
su $(id -u -n) -c 'cd modules; make clean all PLATFORM=photon'
"
}

main
