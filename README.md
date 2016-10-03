[![Build Status](https://travis-ci.org/Ragnaroek/rust-trellis.svg?branch=master)](https://travis-ci.org/Ragnaroek/rust-trellis)
[![Coverage Status](https://coveralls.io/repos/github/Ragnaroek/rust-trellis/badge.svg)](https://coveralls.io/github/Ragnaroek/rust-trellis)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)](https://github.com/Ragnaroek/rust-trellis/blob/master/LICENSE)
[![](http://meritbadge.herokuapp.com/trellis)](https://crates.io/crates/trellis)

# Adafruit Trellis Library for Rust
This is a pure Rust port of the [Adafruit Trellis Python](https://github.com/tdicola/Adafruit_Trellis_Python) and [Adafruit Trellis C++](https://github.com/adafruit/Adafruit_Trellis_Library) libraries.

## Documentation

The documentation for the current version can be found here: [Rust Trellis Documenation](http://ragnaroek.github.io/rust-trellis/trellis/)

## Usage

To see what using the library looks like, let's have a look at
the example program from the project. The example program turns
the LED on or off if the corresponding button on the Trellis is pressed.

```rust
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
```

You first need to create a concrete host device the Trellis is connected to.
Since I only own a Raspberry Pi 2 B+ this is the only one that comes with
the library. Implementing you own device should be straightforward with
`RaspberryPiBPlus` as a basis.

You can now create a new Trellis with `new` and the previously
create host device object. Before you interact with
the Trellis you have to call `init` (exactly once).

After `init` you can now manipulate the LED states with calls
to `set_led` and `clear_led`. The current LED state can be queried
with `is_led_set`. The LEDs are addressed in rows (from R0 to R3) and columns (from
  A to D). The addressing layout is as follows:

```
[(A,R0),....,(D,R0)]
[..................]----- (cables)
[..................]-----
[(A,R3).....,(D,R3)]
```

After manipulating the LED state you have to write the state to the device with
a call to `write_display`. This will turn the LEDs on the Trellis on/off according
to the previously set LED state.

For reading the button state you can register a callback function on
the `button_evt_loop` function. Note that this functions does only terminate
if the supplied callback function returns `true`. The button state is constantly
queried in the `button_evt_loop` with a resolution of 30ms (currently fixed).

The callback function will receive a list of pressed buttons in the `EventButton`
object. The function will receive the button press event only once, regardless how long
the button is pressed by the user.
