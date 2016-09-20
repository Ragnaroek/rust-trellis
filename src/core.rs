extern crate i2cdev;

use std;
use std::time::Duration;
use super::devices::I2CMasterDevice;

const NUM_LEDS: usize = 16;

type LedVec = [bool; NUM_LEDS];

static LED_ADDRESSES : [u8; NUM_LEDS] = [
      0x3A, 0x37, 0x35, 0x34,
      0x28, 0x29, 0x23, 0x24,
      0x16, 0x1B, 0x11, 0x10,
      0x0E, 0x0D, 0x0C, 0x02
];

static BUTTON_ADDRESSES : [u8; NUM_LEDS] = [
      0x07, 0x04, 0x02, 0x22,
      0x05, 0x06, 0x00, 0x01,
      0x03, 0x10, 0x30, 0x21,
      0x13, 0x12, 0x11, 0x31
];

/// A column in the trellis grid.
/// See the project readme for a scheme of the trellis orientation.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Col {
    A, B, C, D
}

/// A row in the trellis grid.
/// See the project readme for a scheme of the trellis orientation.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Row {
    R0, R1, R2, R3
}

/// A particular button and LED combination in the grid.
/// Inconsistently used in the trellis implementation
/// at the moment.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct LedButton {
    pub col: Col,
    pub row: Row
}

/// Describes a button event that occurred. At the moment only button press
/// is implemented.
///
/// Since the buttons on the trellis can be pressed simultaneouly a list
/// of buttons is supplied. The order in this list is undefined.
#[derive(Debug)]
pub struct ButtonEvent {
    pub buttons_pressed: Vec<LedButton>
}


impl ButtonEvent {
    /// Returns a empty event (nothing happend).
    pub fn empty() -> ButtonEvent {
        return ButtonEvent{buttons_pressed: vec![]}
    }
}

/// The callback type for the button_evt_loop (see the Trellis struct for details).
pub type EventLoopHandler = Box<FnMut(&mut Trellis, ButtonEvent) -> bool>;

// Helper methods

fn row_to_num(row: Row) -> u8 {
    match row {
        Row::R0 => 0,
        Row::R1 => 1,
        Row::R2 => 2,
        Row::R3 => 3
    }
}

fn num_to_row(num: usize) -> Row {
    match num {
        0 => Row::R0,
        1 => Row::R1,
        2 => Row::R2,
        3 => Row::R3,
        _ => panic!("illegal row")
    }
}

fn col_to_num(col: Col) -> u8 {
    match col {
        Col::A => 0,
        Col::B => 1,
        Col::C => 2,
        Col::D => 3,
    }
}

fn num_to_col(num: usize) -> Col {
    match num {
        0 => Col::A,
        1 => Col::B,
        2 => Col::C,
        3 => Col::D,
        _ => panic!("illegal column")
    }
}

fn num_to_led_button(num: usize) -> LedButton {
    let col_num = num % 4;
    let row_num = num / 4;
    return LedButton{col: num_to_col(col_num), row: num_to_row(row_num)};
}

fn led_index(col:Col, row:Row) -> usize {
    return (row_to_num(row)*4 + col_to_num(col)) as usize;
}

fn to_button_state(dev_data: Vec<u8>) -> LedVec {
    let mut result = [false; NUM_LEDS];
    for i in 0..NUM_LEDS {
        let addr = BUTTON_ADDRESSES[i];
        result[i] = (dev_data[(addr >> 4) as usize] & (1 << (addr & 0x0F))) > 0;
    }
    return result;
}

fn button_event(old: LedVec, new: LedVec) -> ButtonEvent {
    let mut pressed = Vec::new();
    for i in 0..NUM_LEDS {
        if new[i] && !old[i] {
            pressed.push(num_to_led_button(i));
        }
    }
    return ButtonEvent{buttons_pressed:pressed};
}

// Trellis impl

pub struct Trellis {
    display_buffer: LedVec,
    button_state: LedVec,
    device : Box<I2CMasterDevice>
}

impl Trellis {

    pub fn new(dev: Box<I2CMasterDevice>) -> Trellis {
        return Trellis { display_buffer: [false; NUM_LEDS],
                         button_state: [false; NUM_LEDS],
                         device: dev};
    }

    pub fn init(&mut self) {
        let empty_array:[u8;0] = [];
        self.device.write_block(0x21, &empty_array).unwrap();
        // set blink off
        self.device.write_block(0x80 | 0x01 | 0 << 1, &empty_array).unwrap();
        //set brightness to max
        self.device.write_block(0xE0 | 15, &empty_array).unwrap();
        //turn interrupt on
        self.device.write_block(0xA1, &empty_array).unwrap();
    }

    pub fn set_led(&mut self, col:Col, row: Row) {
        self.display_buffer[led_index(col, row)] = true;
    }

    pub fn clear_led(&mut self, col:Col, row:Row) {
        self.display_buffer[led_index(col, row)] = false;
    }

    pub fn is_led_set(&mut self, col:Col, row: Row) -> bool {
        return self.display_buffer[led_index(col, row)];
    }

    pub fn write_display(&mut self) {
        let mut data:[u16; 8] = [0; 8];
        for l in 0..NUM_LEDS {
            let led_addr = LED_ADDRESSES[l];
            if self.display_buffer[l] {
                data[(led_addr >> 4) as usize] |= 1 << (led_addr & 0x0F);
            } else {
                data[(led_addr >> 4) as usize] &= !(1 << (led_addr & 0x0F));
            }
        }

        let mut w:[u8; 16] = [0; 16];
        for i in 0..8 {
            w[i*2] = (data[i] & 0xFF) as u8;
            w[i*2+1] = (data[i] >> 8) as u8;
        }

        self.device.write_block(0x0, &w).unwrap();
    }

    /* Start the button read event loop. This function does not terminate and can
     * only be stopped by the supplied event handler.
     */
    pub fn button_evt_loop(&mut self, mut hnd: EventLoopHandler) {
        loop {
            let new_button_state = to_button_state(self.device.read_block(0x40, 6).unwrap());
            let event = button_event(self.button_state, new_button_state);

            self.button_state = new_button_state;

            let handler_result = hnd(self, event);
            if handler_result {
                break;
            }
            std::thread::sleep(Duration::from_millis(30));
        }
    }
}
