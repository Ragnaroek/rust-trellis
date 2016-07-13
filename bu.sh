# bu.sh (build-and-upload)

PI="pi@pi"

./build.sh
rsync ./target/arm-unknown-linux-gnueabihf/debug/trellis_demo $PI:~
