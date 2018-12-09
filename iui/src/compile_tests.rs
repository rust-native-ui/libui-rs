//! Examples of unsound code that IUI statically prevents from compiling.
//!
//! Here, we attempt to place use-after-free some callbacks.
//!
//! ```compile_fail
//! let ev = iui::UI::init().unwrap();
//!
//! {
//!     let v = vec![1, 2, 3, 4];
//!     ev.queue_main(|| {
//!         for i in &v {
//!             println!("{}", i);
//!         }
//!     });
//! }
//!
//! ev.quit();
//! ev.main();
//! ```
//!
//! ```compile_fail
//! let ev = iui::UI::init().unwrap();
//!
//! {
//!     let v = vec![1, 2, 3, 4];
//!     ev.on_should_quit(|| {
//!         for i in &v {
//!             println!("{}", i);
//!         }
//!     });
//! }
//!
//! ev.quit();
//! ev.main();
//! ```
//!
//! ```
//! let ev = iui::UI::init().unwrap();
//!
//! let v = vec![1, 2, 3, 4];
//! ev.on_should_quit(move || {
//!     for i in &v {
//!         println!("{}", i);
//!     }
//! });
//!
//! ev.quit();
//! ev.main();
//! ```
