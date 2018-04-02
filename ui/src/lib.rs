//! Rust bindings to `libui`, a wrapper library for native(ish) GUI libraries - Win32API on Windows, Cocoa on Mac OS X, and GTK+ on Linux and elsewhere.
//! This library exposes the a Rusty procedural interface to the "Least Common Denominator" of GUI widgets. They are all available on all supported platforms.
//!
//! To use this library, import it in your `Cargo.toml`:
//!
//! ```ignore
//! ui = { git = "https://github.com/LeoTindall/libui-rs" }
//! ```
//!
//! `libui` requires some global initialization and, thus, deinitialization. This is implemented as the [`UI`](struct.UI.html) RAII guard.
//!
//! Buttons, text entry boxes, and all other GUI elements are implemented as "controls".
//! They can only be displayed as children of a [`Window`](window/struct.Window.html).
//!
//! # Example
//!
//! ```
//! extern crate ui;
//! use ui::prelude::*;
//! use ui::controls::{BoxControl, Button};
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
//!     // Adding this callback means that when this window closes,
//!     // the `ui::main` function returns.
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
//!     let mut quit_button = Button::new("Quit");
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

pub use ffi_utils::Text;
pub use ui::{InitError, UI};
pub use window::Window;
pub use image::Image;

#[macro_use]
pub mod controls;
pub mod draw;
pub mod ffi_utils;
pub mod menus;
pub mod prelude;
mod ui;
pub mod window;
mod image;