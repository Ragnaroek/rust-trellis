use super::devices::I2CMasterDevice;
use std::io::Result;

pub struct MockDevice {
}

impl MockDevice {
    pub fn new() -> MockDevice {
        return MockDevice {};
    }
}

impl I2CMasterDevice for MockDevice {
    fn write_block(&mut self, _register: u8, _values: &[u8]) -> Result<()> {
        return Ok(());
    }
}
