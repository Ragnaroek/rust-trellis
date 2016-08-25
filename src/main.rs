extern crate i2cdev;
extern crate trellis;

use std::time::Duration;
use std::boxed::Box;
use trellis::core::Trellis;
use trellis::devices::RaspberryPiBPlus;
use trellis::core::Row;
use trellis::core::Col;

fn main() {
    let pi_dev = RaspberryPiBPlus::new();
    let mut trellis = Trellis::new(Box::new(pi_dev));
    trellis.init();
    trellis.set_led(Col::A, Row::R0);
    trellis.set_led(Col::B, Row::R1);
    trellis.set_led(Col::C, Row::R2);
    trellis.set_led(Col::D, Row::R3);
    trellis.write_display();

    let mut reads = 0;
    loop {
        trellis.read_switches_test();
        reads = reads+1;
        std::thread::sleep(Duration::from_millis(100));
        if reads == 600 {
            println!("Ending");
            break;
        }
    }


}
