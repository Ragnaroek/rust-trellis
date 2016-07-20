extern crate i2cdev;

use self::i2cdev::core::I2CDevice;
use self::i2cdev::linux::LinuxI2CDevice;

pub struct Trellis {
    i2c_device : LinuxI2CDevice,
    display_buffer: [bool; 16]
}

pub const DEFAULT_TRELLIS_ADDR: u16 = 0x70;

static LED_ADDRESSES : [u8; 16] = [
      0x3A, 0x37, 0x35, 0x34,
      0x28, 0x29, 0x23, 0x24,
      0x16, 0x1B, 0x11, 0x10,
      0x0E, 0x0D, 0x0C, 0x02];

impl Trellis {

    //TODO Pass in device instead of hard coding device information
    pub fn new() -> Trellis {
        let device = "/dev/i2c-1"; // I2C-Bus 1

        let empty_array:[u8;0] = [];

        let mut i2cdev = LinuxI2CDevice::new(device, DEFAULT_TRELLIS_ADDR).unwrap();
        i2cdev.smbus_process_block(0x21, &empty_array).unwrap();
        println!("Oscillator turned on");

        i2cdev.smbus_process_block(0x80 | 0x01 | 0 << 1, &empty_array).unwrap();
        println!("Set blink off");

        i2cdev.smbus_process_block(0xE0 | 15, &empty_array).unwrap();
        println!("Set brightness to max");

        i2cdev.smbus_process_block(0xA1, &empty_array).unwrap();
        println!("Interrupt turned on");

        return Trellis { display_buffer: [false; 16], i2c_device: i2cdev};
    }

    pub fn set_led(&mut self, led: u8) {
        if led >= 16 {return};
        self.display_buffer[led as usize] = true;
    }

    pub fn write_display(&self) {
        // check initialised
        // write to display with i2c
    }
}
