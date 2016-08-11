extern crate trellis;

use std::boxed::Box;
use trellis::core::Trellis;
use trellis::core::Row;
use trellis::core::Col;
use trellis::mock::MockDevice;

#[test]
fn should_set_led() {

    let mut trellis = new_trellis();
    trellis.set_led(Col::A, Row::R0);
    trellis.set_led(Col::D, Row::R3);
    trellis.set_led(Col::C, Row::R2);

    assert_eq!(trellis.is_led_set(Col::A, Row::R0), true);
    assert_eq!(trellis.is_led_set(Col::D, Row::R3), true);
    assert_eq!(trellis.is_led_set(Col::C, Row::R2), true);
    assert_eq!(trellis.is_led_set(Col::A, Row::R1), false);
}

#[test]
fn should_clear_led() {
    let mut trellis = new_trellis();
    trellis.set_led(Col::B, Row::R2);
    assert_eq!(trellis.is_led_set(Col::B, Row::R2), true);

    trellis.clear_led(Col::B, Row::R2);
    assert_eq!(trellis.is_led_set(Col::B, Row::R2), false);
}

#[test]
fn should_write_display_data_to_i2c() {
    // TODO
}

// helper

/**
 *
 */
fn new_trellis() -> Trellis {
    let mock_dev = MockDevice::new();
    let mut trellis = Trellis::new(Box::new(mock_dev));
    trellis.init();
    return trellis;
}
