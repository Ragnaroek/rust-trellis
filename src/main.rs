extern crate i2cdev;

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

pub const TRELLIS_ADDR: u16 = 0x70;  /// why 0x52, probably 0x70, 0x71 or something (test with i2c device tester)

// I2C Bus: 1 (on raspberry)

// TODO small lib for raspberry i2c devices?

fn main() {
    let device = "/dev/i2c-1"; // I2C-Bus 1

    let mut i2cdev = LinuxI2CDevice::new(device, TRELLIS_ADDR).unwrap();
    i2cdev.smbus_write_byte(0x21).unwrap();
    println!("Oscillator turned on");

    i2cdev.smbus_write_byte(0xE0 | 15).unwrap();
    println!("Set brightness to max");

    i2cdev.smbus_write_byte(0xA1).unwrap();
    println!("Interrupt turned on");

    let mut d:[u16; 8] = [0; 8];
    d[0x3A >> 4] |= 1 << (0x3A & 0x0F);

    let mut w:[u8; 16] = [0; 16];
    for i in 0..8 {
        w[i*2] = (d[i] & 0xFF) as u8;
        print!("{}", w[i*2]);
        print!(",");
        w[i*2+1] = (d[i] >> 8) as u8;
        print!("{}", w[i*2+1]);
        print!(",");
    }
    println!("");

    // TODO write zero byte before, like the C-Code?
    i2cdev.write(&w).unwrap();
    println!("LED 0 turned on");

    println!("Waiting a moment");
    std::thread::sleep_ms(1000);
}
