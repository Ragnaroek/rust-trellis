# cross-compiles with the rust cross-compile docker image https://github.com/Ogeon/rust-on-raspberry-pi
docker run --volume ~/pprojects/rust-trellis:/home/cross/project rust-nightly-pi-cross build
