//! Low-level bindings to API-specific functions for interfacing with foreign controls on Mac OS X.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use ffi::uiControl;
use libc::{c_float, c_int};

#[repr(C)]
pub struct uiDarwinControl {
    pub c: uiControl,
    pub parent: *mut uiControl,
    pub enabled: BOOL,
    pub visible: BOOL,
    pub SyncEnableState: extern "C" fn(*mut uiDarwinControl, c_int),
    pub SetSuperview: extern "C" fn(*mut uiDarwinControl, *mut NSView),
    pub HugsTrailingEdge: extern "C" fn(*mut uiDarwinControl) -> BOOL,
    pub HugsBottom: extern "C" fn(*mut uiDarwinControl) -> BOOL,
    pub ChildEdgeHuggingChanged: extern "C" fn(*mut uiDarwinControl),
    pub HuggingPriority: extern "C" fn(*mut uiDarwinControl, NSLayoutConstraintOrientation)
                                       -> NSLayoutPriority,
    pub SetHuggingPriority: extern "C" fn(*mut uiDarwinControl,
                                          NSLayoutPriority,
                                          NSLayoutConstraintOrientation),
}

#[link(name = "ui")]
extern {
    pub fn uiDarwinControlSyncEnableState(control: *mut uiDarwinControl, state: c_int);
    pub fn uiDarwinControlSetSuperview(control: *mut uiDarwinControl, view: *mut NSView);
    pub fn uiDarwinControlHugsTrailingEdge(control: *mut uiDarwinControl) -> BOOL;
    pub fn uiDarwinControlHugsBottom(control: *mut uiDarwinControl) -> BOOL;
    pub fn uiDarwinControlChildEdgeHuggingChanged(control: *mut uiDarwinControl);
    pub fn uiDarwinControlHuggingPriority(control: *mut uiDarwinControl,
                                          orientation: NSLayoutConstraintOrientation)
                                          -> NSLayoutPriority;
    pub fn uiDarwinControlSetHuggingPriority(control: *mut uiDarwinControl,
                                             priority: NSLayoutPriority,
                                             orientation: NSLayoutConstraintOrientation);
}

pub type BOOL = c_int;

pub enum NSView {}

// This is an NSInteger.
#[repr(i64)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NSLayoutConstraintOrientation {
    Horizontal = 0,
    Vertical = 1,
}

pub type NSLayoutPriority = c_float;

pub const NSLayoutPriorityRequired: NSLayoutPriority = 1000.0;
pub const NSLayoutPriorityDefaultHigh: NSLayoutPriority = 750.0;
pub const NSLayoutPriorityDragThatCanResizeWindow: NSLayoutPriority = 510.0;
pub const NSLayoutPriorityWindowSizeStayPut: NSLayoutPriority = 500.0;
pub const NSLayoutPriorityDragThatCannotResizeWindow: NSLayoutPriority = 490.0;
pub const NSLayoutPriorityDefaultLow: NSLayoutPriority = 250.0;
pub const NSLayoutPriorityFittingSizeCompression: NSLayoutPriority = 50.0;

