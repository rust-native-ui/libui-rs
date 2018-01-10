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
//! use ui::{UI, Window, BoxControl, Button};
//! 
//! fn main() {
//!     // Start up the UI toolkit
//!     let ui = Rc::new(UI::init().unwrap());
//! 
//!     // Create a new window, 200x100, titled "Test Window"
//!     // and put it in an Rc so it can be passed into callback functions.
//!     let main_window = Rc::new(Window::new("Test App", 200, 100, true));
//! 
//!     // Add margins around the edge of the window, making it look much nicer.
//!     main_window.set_margined(true);
//! 
//!     // Adding this callback means that when this window closes, the `ui::main` function returns.
//!     // This should be added to the primary window of any application.
//!     {
//!         let ui = ui.clone();
//!         main_window.on_closing(Box::new(move |_| {
//!             ui.quit();
//!             false
//!         }));
//!     }
//! 
//!     // Create a button that opens a dialog box.
//!     let button = Button::new("Button");
//!     {
//!         // Make a new Rc reference to the main window for this closure.
//!         let main_window = main_window.clone();
//!         // on_clicked runs the given closure when the button is clicked.
//!         // A lot of widgets provide this event, or others like it.
//!         button.on_clicked(Box::new(move |_| {
//!             // msg_box creates a modal dialog with the given title and text
//!             main_window.msg_box("Button", "You clicked the button!");
//!         }));
//!     }
//! 
//!     // Create a button that quits the app.
//!     let quit_button = Button::new("Quit");
//!     {
//!         let ui = ui.clone();
//!         quit_button.on_clicked(Box::new(move |_| { ui.quit(); }));
//!     }
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
//!     // Just for testing, quit the app before it even starts.
//!     // Otherwise the test would never end!
//!     ui.quit();
//! 
//!     // Run the app.
//!     ui.main();
//! }
//! ```

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate ui_sys;

pub use controls::{Area, AreaMouseEvent, AreaDrawParams, AreaKeyEvent, AreaHandler, BoxControl,
                   Button, Checkbox, ColorButton};
pub use controls::{Combobox, EditableCombobox, Control, DateTimePicker, Entry, FontButton, Group,
                   Label};
pub use controls::{MultilineEntry, ProgressBar, RadioButtons, Separator, Slider, Spinbox, Tab};
pub use ffi_utils::Text;
pub use menus::{Menu, MenuItem};
pub use ui::{InitError, UI};
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
