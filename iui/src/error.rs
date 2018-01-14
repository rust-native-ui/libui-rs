//! Error types for this crate.

/// The error type returned by functions in this crate which might fail.
#[derive(Fail, Debug)]
pub enum UIError {
    /// Signifies that the underlying library was unable to properly hook into the platform's GUI APIs.
    #[fail(display = "unable to initialize the underlying system bindings: {}", error)]
    FailedInitError { error: String },
    /// Signifies that an attempt was made to initialize a new instance of the underlying library while
    /// one already existed.
    #[fail(display = "cannot initialize multiple instances of the libui toolkit")]
    MultipleInitError(),
}
