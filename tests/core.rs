extern crate trellis;

use std::boxed::Box;
use trellis::core::Trellis;
use trellis::core::Row;
use trellis::core::Col;
use trellis::core::LedButton;
use trellis::core::ButtonEvent;
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
    let mut trellis = new_trellis_write_only(write_cb);
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
    let mut trellis = new_trellis_write_only(write_cb);
    trellis.set_led(Col::A, Row::R0);
    trellis.set_led(Col::B, Row::R1);
    trellis.set_led(Col::C, Row::R2);
    trellis.set_led(Col::D, Row::R3);
    trellis.write_display();
}

#[test]
fn should_quit_event_loop_if_handler_returns_false() {
    let mut cnt = 0;
    let cb = Box::new(move |_trellis:&mut Trellis,_evt:ButtonEvent| {
        cnt = cnt + 1;
        if cnt > 3 {
            return true;
        }
        return false;
    });

    let mut trellis = new_trellis();
    trellis.button_evt_loop(cb);
    assert!(true); // all good if we terminate
}

#[test]
fn should_detect_button_press() {
    //read_data vec are raw data for button press of Col::C, Row::R2
    let mut trellis = new_trellis_read_only(vec![0,0,0,1,0,0]);

    let cb = Box::new(move |_trellis:&mut Trellis,evt:ButtonEvent| {
        assert_eq!(evt.buttons_pressed, vec![LedButton{col: Col::C, row: Row::R2}]);
        return true;
    });
    trellis.button_evt_loop(cb);
    assert!(true);
}

#[test]
fn should_detect_multiple_button_press() {
    //read_data vec are raw data for button press of Col::C+D, Row::R3 simultaneously
    let mut trellis = new_trellis_read_only(vec![0,2,0,2,0,0]);

    let cb = Box::new(move |_trellis:&mut Trellis,evt:ButtonEvent| {
        assert_eq!(evt.buttons_pressed, vec![LedButton{col: Col::C, row: Row::R3},
                                             LedButton{col: Col::D, row: Row::R3}]);
        return true;
    });
    trellis.button_evt_loop(cb);
    assert!(true);
}

// helper

fn new_trellis() -> Trellis {
    fn noop(_i:i32, _reg:u8, _vals: &[u8]) {};
    return new_trellis_write_only(noop);
}

fn new_trellis_write_only(cb: WriteCb) -> Trellis {
    let mock_dev = MockDevice::new_write_only(cb);
    let mut trellis = Trellis::new(Box::new(mock_dev));
    trellis.init();
    return trellis;
}

fn new_trellis_read_only(data: Vec<u8>) -> Trellis {
    let mock_dev = MockDevice::new_read_only(data);
    let mut trellis = Trellis::new(Box::new(mock_dev));
    trellis.init();
    return trellis;
}
