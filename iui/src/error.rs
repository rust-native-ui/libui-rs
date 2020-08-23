//! Error types for this crate.

/// The error type returned by functions in this crate which might fail.
#[derive(thiserror::Error, Debug)]
pub enum UIError {
    /// Signifies that the underlying library was unable to properly hook into the platform's GUI APIs.
    #[error("unable to initialize the underlying system bindings: {error}")]
    FailedInitError { error: String },
    /// Signifies that an attempt was made to initialize a new instance of the underlying library while
    /// one already existed.
    #[error("cannot initialize multiple instances of the libui toolkit")]
    MultipleInitError(),
    /// Signifies that an attempt was made to remove a tab from a tab group that was out of bounds.
    #[error("cannot remove index {index} from tab group: there are only {n} tabs in the group")]
    TabGroupIndexOutOfBounds { index: i32, n: i32 },
}
