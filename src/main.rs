extern crate i2cdev;

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

pub const TRELLIS_ADDR: u16 = 0x52;

// I2C Bus: 1 (on raspberry)

// TODO small lib for raspberry i2c devices?

fn main() {
    let device = "/dev/i2c-1"; // I2C-Bus 1
    let mut i2cdev = LinuxI2CDevice::new(device, TRELLIS_ADDR).unwrap();

    //TODO read+and print results of smbus calls
    i2cdev.smbus_write_byte(0x21);
    println!("Oscillator turned on");

    i2cdev.smbus_write_byte(0xA1);
    println!("Interrupt turned on");

    i2cdev.smbus_write_byte(0x3A);
    println!("LED 0 turned on");

    println!("Waiting a moment");
    std::thread::sleep_ms(1000);
}
