//! `iui`, the `i`mproved `u`ser `i`nterface crate, provides Rust bindings to `libui`, a wrapper library for native(ish) GUI libraries
//! - Win32API on Windows, Cocoa on Mac OS X, and GTK+ on Linux and elsewhere. This library exposes a Rusty procedural interface to the
//! "Least Common Denominator" of GUI widgets. They are all available on all supported platforms.
//! To use this library, import it in your `Cargo.toml`:
//!
//! ```toml
//! iui = { git = "https://github.com/LeoTindall/libui-rs" }
//! ```
//!
//! Most of the functionality of the crate is exposed via the [UI](struct.UI.html) RAII guard, which handles all initialization and cleanup for the
//! underlying library.

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate libc;
extern crate ui_sys;

mod ui;
mod error;
mod ffi_tools;
pub mod controls;

pub use ui::UI;
pub use error::UIError;

/// Common imports are packaged into this module. It's meant to be glob-imported: `use iui::prelude::*`.
pub mod prelude {
    pub use ui::UI;
    pub use controls::{Window, WindowType};
    pub use controls::{LayoutStrategy};
    pub use controls::{NumericEntry, TextEntry};
}
