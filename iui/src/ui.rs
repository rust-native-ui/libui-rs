use ui_sys;
use error::UIError;
use ffi_tools;

use std::rc::Rc;
use std::marker::PhantomData;
use std::ffi::CStr;
use std::mem;

use controls::Window;

/// RAII guard for the UI; when dropped, it uninits libUI.
struct UIToken {
    // This PhantomData prevents UIToken from being Send and Sync
    _pd: PhantomData<*mut ()>,
}

impl Drop for UIToken {
    fn drop(&mut self) {
        assert!(
            ffi_tools::is_initialized(),
            "Attempted to uninit libUI in UIToken destructor when libUI was not initialized!"
        );
        unsafe {
            Window::destroy_all_windows();
            ui_sys::uiUninit();
            ffi_tools::unset_initialized();
        }
    }
}

/// A handle to user interface functionality.
#[derive(Clone)]
pub struct UI {
    token: Rc<UIToken>,
}

impl UI {
    /// Initializes the underlying UI bindings, producing a [`UI`](struct.UI.html) struct which can be used
    /// to actually build your user interface. This is a reference counted type; clone it
    /// to get an additional reference that can be passed to, e.g., callbacks.
    ///
    /// Only one libUI binding can be active at once; if multiple instances are detected,
    /// this function will return a [`MultipleInitError`](enum.UIError.html#variant.MultipleInitError).
    ///
    /// ```
    /// # use iui::UI;
    /// {
    ///     let ui1 = UI::init().unwrap();
    ///
    ///     // This will fail because there is already an instance of UI.
    ///     let ui2 = UI::init();
    ///     assert!(ui2.is_err());
    ///
    ///     // ui1 dropped here.
    /// }
    /// let ui3 = UI::init().unwrap();
    /// ```
    ///
    /// If libUI cannot initialize its hooks into the platform bindings, this function will
    /// return a [`FailedInitError`](enum.UIError.html#variant.FailedInitError) with the description of the problem.
    pub fn init() -> Result<UI, UIError> {
        if ffi_tools::is_initialized() {
            return Err(UIError::MultipleInitError {});
        };

        unsafe {
            // Create the magic value needed to init libUI
            let mut init_options = ui_sys::uiInitOptions {
                Size: mem::size_of::<ui_sys::uiInitOptions>(),
            };

            // Actually start up the library's functionality
            let err = ui_sys::uiInit(&mut init_options);
            if err.is_null() {
                // Success! We can safely give the user a token allowing them to do UI things.
                ffi_tools::set_initialized();
                Ok(UI {
                    token: Rc::new(UIToken { _pd: PhantomData }),
                })
            } else {
                // Error occurred; copy the string describing it, then free that memory.
                let error_string = CStr::from_ptr(err).to_string_lossy().into_owned();
                ui_sys::uiFreeInitError(err);
                Err(UIError::FailedInitError {
                    error: error_string,
                })
            }
        }
    }

    /// Hands control of this thread to the UI toolkit, allowing it to display the UI and respond to events. Does not return until the UI [quit](struct.UI.html#method.quit)s.
    pub fn main(&self) {
        unsafe { ui_sys::uiMain() }
    }

    /// Running this function causes the UI to quit, exiting from [main](struct.UI.html#method.main) and no longer showing any widgets.
    pub fn quit(&self) {
        unsafe { ui_sys::uiQuit() }
    }
}
