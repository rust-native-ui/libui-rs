//! Utilities to manage the state of the interface to the libUI bindings.
use libc::c_void;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

/// Set the global flag stating that libUI is initialized.
///
/// # Unsafety
/// If this is called when the library is not, in fact, initialized,
/// the program will be placed in an undefined state.
///
/// # Panics
/// Panics if called when libUI is already marked as initialized.
pub unsafe fn set_initialized() {
    assert!(!INITIALIZED.swap(true, Ordering::SeqCst),
        "Tried to initialize libUI when it was already initialized. Aborting because this is an unsafe situation.");
}

/// Set the global flag stating that libUI is no longer initialized.
///
/// # Unsafety
/// If this is called when the library is actually still initialized,
/// the program could try to create a new instance, violating the library's
/// invariants and likely causing a segfault.
pub unsafe fn unset_initialized() {
    INITIALIZED.store(false, Ordering::SeqCst);
}

/// Retrieve the global flag indicating whether libUI is initialized.
pub fn is_initialized() -> bool {
    INITIALIZED.load(Ordering::SeqCst)
}

// Transmute a void-void callback into a Box<Box<FnMut()>>> and call it
pub extern "C" fn void_void_callback(data: *mut c_void) {
    unsafe { mem::transmute::<*mut c_void, Box<Box<FnMut()>>>(data)() }
}
