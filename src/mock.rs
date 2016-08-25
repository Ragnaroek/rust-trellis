use super::devices::I2CMasterDevice;
use std::io::Result;

pub type WriteCb = fn(i32, u8, &[u8]) -> ();

pub struct MockDevice {
    write_block_counter: i32,
    write_block_cb: WriteCb

    // TODO add write_block_cb: fn(i32) -> void and call counter to MockDevice
}

impl MockDevice {
    pub fn new(write_block_cb: WriteCb) -> MockDevice {
        return MockDevice {write_block_cb: write_block_cb, write_block_counter: 0};
    }
}

impl I2CMasterDevice for MockDevice {
    fn write_block(&mut self, register: u8, values: &[u8]) -> Result<()> {
        let cb = self.write_block_cb;
        cb(self.write_block_counter, register, values);
        self.write_block_counter += 1;
        return Ok(());
    }

    fn read_block(&mut self, _register: u8) -> Result<Vec<u8>> {
        // TODO impl callback
        return Ok(Vec::new());
    }
}
