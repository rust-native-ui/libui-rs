//! `iui`, the `i`mproved `u`ser `i`nterface crate, provides Rust bindings to `libui`, a wrapper library for native(ish) GUI libraries
//! - Win32API on Windows, Cocoa on Mac OS X, and GTK+ on Linux and elsewhere. This library exposes a Rusty procedural interface to the
//! "Least Common Denominator" of GUI widgets. They are all available on all supported platforms.
//!
//! To use the library, add the following to your `Cargo.toml`:
//!
//! ```toml
//! "iui" = "0.2"
//! ```
//!
//! Most of the functionality of the crate is exposed via the [UI](struct.UI.html) RAII guard, which handles all initialization and cleanup for the
//! underlying library.
//!
//! After initialization, all the functionality used for creating actual UIs is in the [`controls`](controls/index.html) module.
//!
//! Fine-grained control of the event loop is avilable via the [`EventLoop`](struct.EventLoop.html) struct.
//!
//! For code examples, see the [basic](https://github.com/LeoTindall/libui-rs/blob/master/iui/examples/basic.rs) and
//! [advanced](https://github.com/LeoTindall/libui-rs/blob/master/iui/examples/inputs.rs) examples or the
//! [file editing](https://github.com/LeoTindall/libui-rs/blob/master/iui/examples/files.rs) example.

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate libc;
extern crate ui_sys;

mod ui;
mod error;
mod ffi_tools;
pub mod menus;
pub mod controls;
pub mod draw;
pub mod image;

pub use ui::{UI, EventLoop};
pub use error::UIError;

/// Common imports are packaged into this module. It's meant to be glob-imported: `use iui::prelude::*`.
pub mod prelude {
    pub use ui::UI;
    pub use controls::{Window, WindowType};
    pub use controls::{LayoutStrategy};
    pub use controls::{NumericEntry, TextEntry};
}
