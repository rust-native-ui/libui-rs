//! Rust bindings to `libui`:
//!
//!     https://github.com/andlabs/libui
//!
//! Copyright © 2016 Mozilla Foundation

extern crate libc;

pub use controls::{BoxControl, Button, Control, Entry};
pub use ffi_utils::Text;
pub use ui::{InitError, init, main, on_should_quit, queue_main, quit, uninit};
pub use windows::Window;

mod controls;
pub mod ffi;
pub mod ffi_utils;
mod ui;
mod windows;

