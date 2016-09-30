//! The trellis lib provides functionality to control the Adafruit Trellis
//! from Rust. It allows to control the LEDs and read button press events.
//! For details have a look at the Trellis struct in the core package.
#![crate_name = "trellis"]
#![crate_type = "lib"]

pub mod core;
pub mod devices;
pub mod mock;
