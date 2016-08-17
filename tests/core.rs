extern crate trellis;

use std::boxed::Box;
use trellis::core::Trellis;
use trellis::core::Row;
use trellis::core::Col;
use trellis::mock::MockDevice;
use trellis::mock::WriteCb;

const FIRST_NON_INIT_CALL_ID:i32 = 4;

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
fn should_write_inital_display_data_to_i2c() {
    fn write_cb(i:i32, reg: u8, values: &[u8]) {
        if i >= FIRST_NON_INIT_CALL_ID {
            let expected_vals:[u8; 16] = [0; 16];
            assert_eq!(i, FIRST_NON_INIT_CALL_ID);
            assert_eq!(reg, 0);
            assert_eq!(values, expected_vals);
        }
    }
    let mut trellis = new_trellis_with_cb(write_cb);
    trellis.write_display();
}

#[test]
fn should_write_display_data_to_i2c() {
    fn write_cb(i:i32, reg: u8, values: &[u8]) {
        if i >= FIRST_NON_INIT_CALL_ID {
            let expected_vals:[u8; 16] = [4, 0, 2, 0, 0, 2, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0];
            assert_eq!(i, FIRST_NON_INIT_CALL_ID);
            assert_eq!(reg, 0);
            assert_eq!(values, expected_vals);
        }
    }
    let mut trellis = new_trellis_with_cb(write_cb);
    trellis.set_led(Col::A, Row::R0);
    trellis.set_led(Col::B, Row::R1);
    trellis.set_led(Col::C, Row::R2);
    trellis.set_led(Col::D, Row::R3);
    trellis.write_display();
}

// helper

fn new_trellis() -> Trellis {
    fn noop(_i:i32, _reg:u8, _vals: &[u8]) {};
    return new_trellis_with_cb(noop);
}

fn new_trellis_with_cb(cb: WriteCb) -> Trellis {
    let mock_dev = MockDevice::new(cb);
    let mut trellis = Trellis::new(Box::new(mock_dev));
    trellis.init();
    return trellis;
}
