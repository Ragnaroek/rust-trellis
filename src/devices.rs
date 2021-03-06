extern crate i2cdev;

use self::i2cdev::core::I2CDevice;
use self::i2cdev::linux::LinuxI2CDevice;
use self::i2cdev::linux::LinuxI2CError;
use std::io::Result;
use std::io;
use std::result;

/// Host devices that are connected to the Trellis. The Trellis is the slave.
/// This trate is a small abstraction over the full I2C
/// since we need only a very small subset.
/// This trate exists also to make the I2C communciation testable (see also MockDevice
/// that implements this trait).
pub trait I2CMasterDevice {
    fn write_block(&mut self, register: u8, values: &[u8]) -> Result<()>;
    fn read_block(&mut self, register: u8, len: u8) -> Result<Vec<u8>>;
}


/// A concrete device for using the Trellis
/// with a Raspberry Pi 2 B+.
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
        return convert_to_io_error(result);
    }

    fn read_block(&mut self, register: u8, len: u8) -> Result<Vec<u8>> {
        let result = self.i2c_device.smbus_read_i2c_block_data(register, len);
        return convert_to_io_error(result);
    }
}

// END Raspberry Pi B+

fn convert_to_io_error<T>(result: result::Result<T, LinuxI2CError>) -> Result<T> {
    match result {
        Ok(o) => Ok(o),
        Err(e) => Err(io::Error::from(e))
    }
}
