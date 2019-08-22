use error::UIError;
use ffi_tools;
use std::os::raw::{c_int, c_void};
use ui_sys;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

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
    /// Be aware the Cocoa (GUI toolkit on Mac OS) requires that the _first thread spawned_ controls
    /// the UI, so do _not_ spin off your UI interactions into an alternative thread. You're likely to
    /// have problems on Mac OS.
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

    /// Hands control of this thread to the UI toolkit, allowing it to display the UI and respond to events.
    /// Does not return until the UI [quit](struct.UI.html#method.quit)s.
    ///
    /// For more control, use the `EventLoop` struct.
    pub fn main(&self) {
        self.event_loop().run(self);
    }

    /// Returns an `EventLoop`, a struct that allows you to step over iterations or events in the UI.
    pub fn event_loop(&self) -> EventLoop {
        unsafe { ui_sys::uiMainSteps() };
        return EventLoop {
            _pd: PhantomData,
            callback: None,
        };
    }

    /// Running this function causes the UI to quit, exiting from [main](struct.UI.html#method.main) and no longer showing any widgets.
    ///
    /// Run in every window's default `on_closing` callback.
    pub fn quit(&self) {
        unsafe { ui_sys::uiQuit() }
    }

    /// Queues a function to be executed on the GUI thread when next possible. Returns
    /// immediately, not waiting for the function to be executed.
    ///
    /// # Example
    ///
    /// ```
    /// use iui::prelude::*;
    ///
    /// let ui = UI::init().unwrap();
    ///
    /// ui.queue_main(|| { println!("Runs first") } );
    /// ui.queue_main(|| { println!("Runs second") } );
    /// ui.quit();
    /// ```
    pub fn queue_main<'ctx, F: FnMut() + 'ctx>(&'ctx self, callback: F) {
        unsafe {
            let mut data: Box<Box<FnMut()>> = Box::new(Box::new(callback));
            ui_sys::uiQueueMain(
                None,
                &mut *data as *mut Box<FnMut()> as *mut c_void,
            );
            mem::forget(data);
        }
    }

    /// Set a callback to be run when the application quits.
    pub fn on_should_quit<'ctx, F: FnMut() + 'ctx>(&'ctx self, callback: F) {
        unsafe {
            let mut data: Box<Box<FnMut()>> = Box::new(Box::new(callback));
            ui_sys::uiOnShouldQuit(
                None,
                &mut *data as *mut Box<FnMut()> as *mut c_void,
            );
            mem::forget(data);
        }
    }
}

/// Provides fine-grained control over the user interface event loop, exposing the `on_tick` event
/// which allows integration with other event loops, custom logic on event ticks, etc.
/// Be aware the Cocoa (GUI toolkit on Mac OS) requires that the _first thread spawned_ controls
/// the UI, so do _not_ spin off your UI interactions into an alternative thread. You're likely to
/// have problems on Mac OS.

pub struct EventLoop<'s> {
    // This PhantomData prevents UIToken from being Send and Sync
    _pd: PhantomData<*mut ()>,
    // This callback gets run during "run_delay" loops.
    callback: Option<Box<FnMut() + 's>>,
}

impl<'s> EventLoop<'s> {
    /// Set the given callback to run when the event loop is executed.
    /// Note that if integrating other event loops you should consider
    /// the potential benefits and drawbacks of the various run modes.
    pub fn on_tick<'ctx, F: FnMut() + 's + 'ctx>(&'ctx mut self, _ctx: &'ctx UI, callback: F) {
        self.callback = Some(Box::new(callback));
    }

    /// Executes a tick in the event loop, returning immediately.
    /// The `on_tick` callback is executed after the UI step.
    ///
    /// Returns `true` if the application should continue running, and `false`
    /// if it should quit.
    pub fn next_tick(&mut self, _ctx: &UI) -> bool {
        let result = unsafe { ui_sys::uiMainStep(false as c_int) == 1 };
        if let Some(ref mut c) = self.callback {
            c();
        }
        result
    }

    /// Hands control to the event loop until the next UI event occurs.
    /// The `on_tick` callback is executed after the UI step.
    ///
    /// Returns `true` if the application should continue running, and `false`
    /// if it should quit.
    pub fn next_event_tick(&mut self, _ctx: &UI) -> bool {
        let result = unsafe { ui_sys::uiMainStep(true as c_int) == 1 };
        if let Some(ref mut c) = self.callback {
            c();
        }
        result
    }

    /// Hands control to the event loop until [`UI::quit()`](struct.UI.html#method.quit) is called,
    /// running the callback given with `on_tick` after each UI event.
    pub fn run(&mut self, ctx: &UI) {
        loop {
            if !self.next_event_tick(ctx) {
                break;
            }
        }
    }

    /// Hands control to the event loop until [`UI::quit()`](struct.UI.html#method.quit) is called,
    /// running the callback given with `on_tick` approximately every
    /// `delay` milliseconds.
    pub fn run_delay(&mut self, ctx: &UI, delay_ms: u32) {
        loop {
            if !self.next_tick(ctx) {
                break;
            }
        }
        sleep(Duration::new(0, delay_ms * 1000000))
    }
}
