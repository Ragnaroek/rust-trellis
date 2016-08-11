extern crate i2cdev;

use self::i2cdev::core::I2CDevice;
use self::i2cdev::linux::LinuxI2CDevice;
use std::io::Result;
use std::io;

/*
 * Host devices that are connected to the Trellis.
 * Trellis is the slave.
 * We only need a small subset of I2C to talk to
 * thre Trellis.
 */
pub trait I2CMasterDevice {
    fn write_block(&mut self, register: u8, values: &[u8]) -> Result<()>;
}


// Raspberry Pi B+ Device impl
pub struct RaspberryPiBPlus {
    i2c_device : LinuxI2CDevice,
}

impl RaspberryPiBPlus {
    pub fn new() -> RaspberryPiBPlus {
        let i2cdev = LinuxI2CDevice::new("/dev/i2c-1", 0x70).unwrap();
        return RaspberryPiBPlus {i2c_device: i2cdev};
    }
}

impl I2CMasterDevice for RaspberryPiBPlus {
    fn write_block(&mut self, register: u8, values: &[u8]) -> Result<()> {
        let result = self.i2c_device.smbus_process_block(register, values);
        match result {
            Ok(o) => Ok(o),
            Err(e) => Err(io::Error::from(e))
        }
    }
}

// END RaspberryPiBPlus
