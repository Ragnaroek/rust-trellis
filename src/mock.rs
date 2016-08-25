use super::devices::I2CMasterDevice;
use std::io::Result;

pub type WriteCb = fn(i32, u8, &[u8]) -> ();

pub struct MockDevice {
    write_block_counter: i32,
    write_block_cb: WriteCb,

    read_block_data: Vec<u8>
}

impl MockDevice {
    pub fn new_write_only(write_block_cb: WriteCb) -> MockDevice {
        return MockDevice::new(write_block_cb, vec![0; 6])
    }

    pub fn new_read_only(read_block_data: Vec<u8>) -> MockDevice {
        fn noop(_i:i32, _reg:u8, _vals: &[u8]) {};
        return MockDevice::new(noop, read_block_data);
    }

    pub fn new(write_block_cb: WriteCb, read_block_data: Vec<u8>) -> MockDevice {
        return MockDevice {write_block_cb: write_block_cb,
                           write_block_counter: 0,
                           read_block_data: read_block_data};
    }
}

impl I2CMasterDevice for MockDevice {
    fn write_block(&mut self, register: u8, values: &[u8]) -> Result<()> {
        let cb = self.write_block_cb;
        cb(self.write_block_counter, register, values);
        self.write_block_counter += 1;
        return Ok(());
    }

    fn read_block(&mut self, _register: u8, _len: u8) -> Result<Vec<u8>> {
        return Ok(self.read_block_data.clone());
    }
}
