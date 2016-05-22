//! Rust bindings to `libui`:
//!
//!     https://github.com/andlabs/libui
//!
//! Copyright © 2016 Mozilla Foundation

extern crate libc;

pub use controls::{BoxControl, Button, Checkbox, Combobox, Control, DateTimePicker, Entry, Group};
pub use controls::{Label, MultilineEntry, ProgressBar, RadioButtons, Separator, Slider, Spinbox};
pub use controls::{Tab};
pub use ffi_utils::Text;
pub use menus::{Menu, MenuItem};
pub use ui::{InitError, init, main, msg_box, msg_box_error, on_should_quit, open_file, queue_main};
pub use ui::{quit, save_file, uninit};
pub use windows::Window;

mod controls;
pub mod ffi;
pub mod ffi_utils;
mod menus;
mod ui;
mod windows;

