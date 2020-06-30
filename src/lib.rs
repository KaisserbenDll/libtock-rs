#![cfg_attr(not(test), no_std)]
// If you get this error while builing :
//no global memory allocator found but one is required;
// link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait.
// Remeber to use FEATURES=alloc while building !!

extern crate alloc;

pub mod adc;
pub mod ble_composer;
pub mod ble_parser;
pub mod buttons;
pub mod console;
pub mod debug;
pub mod drivers;
pub mod electronics;
pub mod executor;
pub mod futures;
pub mod gpio;
pub mod hmac;
pub mod leds;
pub mod result;
pub mod rng;
pub mod sensors;
pub mod simple_ble;
pub mod temperature;
pub mod timer;
pub mod ipc_server;
pub mod ipc_client;

pub use drivers::retrieve_drivers;
pub use libtock_codegen::main;
pub use libtock_core::*;
