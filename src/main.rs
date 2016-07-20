extern crate i2cdev;
extern crate trellis;

use std::time::Duration;
use trellis::core::Trellis;


// fn turn_on_light(dev: &mut LinuxI2CDevice, i:usize) {
//     let led_addr = LED_ADDRESSES[i];
//     let mut data:[u16; 8] = [0; 8];
//     data[(led_addr >> 4) as usize] |= 1 << (led_addr & 0x0F);
//
//     let mut w:[u8; 16] = [0; 16];
//     for i in 0..8 {
//         w[i*2] = (data[i] & 0xFF) as u8;
//         print!("{}", w[i*2]);
//         print!(",");
//         w[i*2+1] = (data[i] >> 8) as u8;
//         print!("{}", w[i*2+1]);
//         print!(",");
//     }
//     println!("");
//
//     dev.smbus_process_block(0x0, &w).unwrap();
//     println!("LED {} turned on", i);
// }
// fn main_old() {
//
//
//     turn_on_light(&mut i2cdev, 0);
//     turn_on_light(&mut i2cdev, 1);
//
//     println!("Waiting a moment");
//     std::thread::sleep(Duration::from_millis(1000));
// }

fn main() {
    let mut trellis = Trellis::new();
}
