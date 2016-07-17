# bu.sh (build-and-upload)

PI="pi@192.168.0.8"

./build.sh
rsync ./target/arm-unknown-linux-gnueabihf/debug/trellis_demo $PI:~
