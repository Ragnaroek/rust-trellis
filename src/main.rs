extern crate i2cdev;
extern crate trellis;

use std::boxed::Box;
use trellis::core::Trellis;
use trellis::core::Col;
use trellis::core::Row;
use trellis::core::ButtonEvent;
use trellis::devices::RaspberryPiBPlus;

fn main() {
    let pi_dev = RaspberryPiBPlus::new();

    let mut trellis = Trellis::new(Box::new(pi_dev));

    trellis.init();
    trellis.set_led(Col::A, Row::R0);
    trellis.write_display();

    let cb = Box::new(|trellis:&mut Trellis, evt:ButtonEvent| {
        if evt.buttons_pressed.len() > 0 {
            println!("pressed button {:?}", evt);
        }
        for button in evt.buttons_pressed {
            if trellis.is_led_set(button.col, button.row) {
                trellis.clear_led(button.col, button.row);
            } else {
                trellis.set_led(button.col, button.row);
            }
        }
        trellis.write_display();
        return false;
    });
    trellis.button_evt_loop(cb);
}
