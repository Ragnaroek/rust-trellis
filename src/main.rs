extern crate i2cdev;
extern crate trellis;

use std::time::Duration;
use trellis::core::Trellis;
use trellis::core::Row;
use trellis::core::Col;

fn main() {
    let mut trellis = Trellis::new();
    trellis.set_led(Col::A, Row::R0);
    trellis.set_led(Col::B, Row::R1);
    trellis.set_led(Col::C, Row::R2);
    trellis.set_led(Col::D, Row::R3);
    trellis.write_display();

    std::thread::sleep(Duration::from_millis(1000));
}
