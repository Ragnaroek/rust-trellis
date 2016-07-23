extern crate i2cdev;

use self::i2cdev::core::I2CDevice;
use self::i2cdev::linux::LinuxI2CDevice;

pub const DEFAULT_TRELLIS_ADDR: u16 = 0x70;

const NUM_LEDS: usize = 16;

static LED_ADDRESSES : [u8; NUM_LEDS] = [
      0x3A, 0x37, 0x35, 0x34,
      0x28, 0x29, 0x23, 0x24,
      0x16, 0x1B, 0x11, 0x10,
      0x0E, 0x0D, 0x0C, 0x02];

pub struct Trellis {
    i2c_device : LinuxI2CDevice,
    display_buffer: [bool; NUM_LEDS]
}

pub enum Col {
    A, B, C, D
}
pub enum Row {
    R0, R1, R2, R3
}

fn row_to_num(row: Row) -> u8 {
    match row {
        Row::R0 => 0,
        Row::R1 => 1,
        Row::R2 => 2,
        Row::R3 => 3
    }
}

fn col_to_num(col: Col) -> u8 {
    match col {
        Col::A => 0,
        Col::B => 1,
        Col::C => 2,
        Col::D => 3
    }
}

impl Trellis {

    pub fn new() -> Trellis {
        let device = "/dev/i2c-1"; // I2C-Bus 1

        let empty_array:[u8;0] = [];

        let mut i2cdev = LinuxI2CDevice::new(device, DEFAULT_TRELLIS_ADDR).unwrap();
        // turn oscillator on
        i2cdev.smbus_process_block(0x21, &empty_array).unwrap();
        // set blink off
        i2cdev.smbus_process_block(0x80 | 0x01 | 0 << 1, &empty_array).unwrap();
        //set brightness to max
        i2cdev.smbus_process_block(0xE0 | 15, &empty_array).unwrap();
        //turn interrupt on
        i2cdev.smbus_process_block(0xA1, &empty_array).unwrap();

        return Trellis { display_buffer: [false; 16], i2c_device: i2cdev};
    }

    pub fn set_led(&mut self, col:Col, row: Row) {
        let led = row_to_num(row)*4 + col_to_num(col);
        self.display_buffer[led as usize] = true;
    }

    pub fn write_display(&mut self) {
        let mut data:[u16; 8] = [0; 8];
        for l in 0..NUM_LEDS {
            let led_addr = LED_ADDRESSES[l];
            if self.display_buffer[l] {
                data[(led_addr >> 4) as usize] |= 1 << (led_addr & 0x0F);
            } else {
                data[(led_addr >> 4) as usize] &= !(1 << (led_addr & 0x0F));
            }
        }

        let mut w:[u8; 16] = [0; 16];
        for i in 0..8 {
            w[i*2] = (data[i] & 0xFF) as u8;
            w[i*2+1] = (data[i] >> 8) as u8;
        }

        self.i2c_device.smbus_process_block(0x0, &w).unwrap();
    }
}
