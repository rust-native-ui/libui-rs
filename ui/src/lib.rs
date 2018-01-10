//! Rust bindings to `libui`.
//!
//! Main C source repository: https://github.com/andlabs/libui
//!
//! Copyright © 2016 Mozilla Foundation
//!
//! # Example
//! 
//! ```
//! extern crate ui;
//! use std::rc::Rc;
//! use ui::{Window, BoxControl, Button};
//! 
//! fn main() {
//!     // Start up the UI toolkit
//!     ui::init(ui::InitOptions);
//! 
//!     // Create a new window, 200x100, titled "Test Window"
//!     // and put it in an Rc so it can be passed into callback functions.
//!     let main_window = Rc::new(Window::new("Test App", 200, 100, true));
//! 
//!     // Add margins around the edge of the window, making it look much nicer.
//!     main_window.set_margined(true);
//! 
//!     // Adding this callback means that when this window closes, 
//!     // the `ui::main` function returns. This should be added to the 
//!     // primary window of any application.
//!     main_window.on_closing(Box::new(|_| {
//!         ui::quit();
//!         false
//!     }));
//! 
//!     // Create a button that opens a dialog box.
//!     let button = Button::new("Button");
//!     // on_clicked runs the given closure when the button is clicked.
//!     // A lot of widgets provide this event, or others like it.
//!     button.on_clicked(Box::new(|_| { println!("Clicked!"); }));
//! 
//!     // Create a button that quits the app.
//!     let mut quit_button = Button::new("Quit");
//!     quit_button.on_clicked(Box::new(|_| { ui::quit(); }));
//! 
//!     // Add a box to lay out controls vertically.
//!     let vbox = BoxControl::new_vertical();
//!     vbox.set_padded(true);
//!     // Put the buttons into the vertical layout.
//!     vbox.append(button.into(), false);
//!     vbox.append(quit_button.into(), false);
//!     // Put the vertical layout into the window.
//!     main_window.set_child(vbox.clone().into());
//! 
//!     // Set the main window (and all its widgets) to visible.
//!     main_window.show();
//! 
//!     // ONLY for testing, quit the app as soon as it starts.
//!     ui::quit();
//! 
//!     // Run the app.
//!     ui::main();
//! 
//!     // Clean up.
//!     ui::uninit();
//! }
//! ```

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate ui_sys;

pub use controls::{Area, AreaMouseEvent, AreaDrawParams, AreaKeyEvent, AreaHandler, BoxControl, Button, Checkbox, ColorButton};
pub use controls::{Combobox, EditableCombobox, Control, DateTimePicker, Entry, FontButton, Group, Label};
pub use controls::{MultilineEntry, ProgressBar, RadioButtons, Separator, Slider, Spinbox, Tab};
pub use ffi_utils::Text;
pub use menus::{Menu, MenuItem};
pub use ui::{InitError, InitOptions, init, main, msg_box, msg_box_error, on_should_quit};
pub use ui::{open_file, queue_main, quit, save_file, uninit};
pub use windows::Window;
pub use image::Image;

#[macro_use]
mod controls;
pub mod draw;
pub mod ffi_utils;
mod menus;
mod ui;
mod windows;
mod image;

