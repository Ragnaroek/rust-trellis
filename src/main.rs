extern crate i2cdev;

use std::time::Duration;

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

pub const TRELLIS_ADDR: u16 = 0x70;  /// why 0x52, probably 0x70, 0x71 or something (test with i2c device tester)

// I2C Bus: 1 (on raspberry)

// TODO small lib for raspberry i2c devices?

const LED_ADDRESSES : [u8; 16] = [
      0x3A, 0x37, 0x35, 0x34,
      0x28, 0x29, 0x23, 0x24,
      0x16, 0x1B, 0x11, 0x10,
      0x0E, 0x0D, 0x0C, 0x02];

fn turn_on_light(dev: &mut LinuxI2CDevice, i:usize) {
    let led_addr = LED_ADDRESSES[i];
    let mut data:[u16; 8] = [0; 8];
    data[(led_addr >> 4) as usize] |= 1 << (led_addr & 0x0F);

    let mut w:[u8; 16] = [0; 16];
    for i in 0..8 {
        w[i*2] = (data[i] & 0xFF) as u8;
        print!("{}", w[i*2]);
        print!(",");
        w[i*2+1] = (data[i] >> 8) as u8;
        print!("{}", w[i*2+1]);
        print!(",");
    }
    println!("");

    dev.smbus_process_block(0x0, &w).unwrap();
    println!("LED {} turned on", i);
}

fn main() {
    let device = "/dev/i2c-1"; // I2C-Bus 1

    let empty_array:[u8;0] = [];

    let mut i2cdev = LinuxI2CDevice::new(device, TRELLIS_ADDR).unwrap();
    i2cdev.smbus_process_block(0x21, &empty_array).unwrap();
    println!("Oscillator turned on");

//HT16K33_BLINK_OFF 0
//#define HT16K33_BLINK_CMD       0x80
//#define HT16K33_BLINK_DISPLAYON 0x01
//#define HT16K33_CMD_BRIGHTNESS 0xE0
    i2cdev.smbus_process_block(0x80 | 0x01 | 0 << 1, &empty_array).unwrap();
    println!("Set blink off");

    i2cdev.smbus_process_block(0xE0 | 15, &empty_array).unwrap();
    println!("Set brightness to max");

    i2cdev.smbus_process_block(0xA1, &empty_array).unwrap();
    println!("Interrupt turned on");

    turn_on_light(&mut i2cdev, 0);
    turn_on_light(&mut i2cdev, 1);

    println!("Waiting a moment");
    std::thread::sleep(Duration::from_millis(1000));
}
