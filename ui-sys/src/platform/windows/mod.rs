//! Low-level bindings to API-specific functions for interfacing with foreign controls on Windows.

#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_char, c_int, c_long, c_ulong, c_uint, c_ushort, c_void, intmax_t, size_t, wchar_t};
use uiControl;

#[repr(C)]
pub struct uiWindowsControl {
    pub c: uiControl,
    pub parent: *mut uiControl,
    pub enabled: BOOL,
    pub visible: BOOL,
    pub SyncEnableState: extern "C" fn(*mut uiWindowsControl, c_int),
    pub SetParentHWND: extern "C" fn(*mut uiWindowsControl, HWND),
    pub MinimumSize: extern "C" fn(*mut uiWindowsControl, *mut intmax_t, *mut intmax_t),
    pub MinimumSizeChanged: extern "C" fn(*mut uiWindowsControl),
    pub LayoutRect: extern "C" fn(*mut uiWindowsControl, *mut RECT),
    pub AssignControlIDZOrder: extern "C" fn(*mut uiWindowsControl, *mut LONG_PTR, *mut HWND),
}

#[link(name = "ui")]
extern {
    pub fn uiWindowsControlSyncEnableState(control: *mut uiWindowsControl, state: c_int);
    pub fn uiWindowsControlSetParentHWND(control: *mut uiWindowsControl, parent: HWND);
    pub fn uiWindowsControlMinimumSize(control: *mut uiWindowsControl,
                                       width: *mut intmax_t,
                                       height: *mut intmax_t);
    pub fn uiWindowsControlMinimumSizeChanged(control: *mut uiWindowsControl);
    pub fn uiWindowsControlLayoutRect(control: *mut uiWindowsControl, rect: *mut RECT);
    pub fn uiWindowsControlAssignControlIDZOrder(control: *mut uiWindowsControl,
                                                 controlID: *mut LONG_PTR,
                                                 insertAfter: *mut HWND);

    pub fn uiWindowsAllocControl(n: size_t, typesig: u32, typenamestr: *const c_char)
                                 -> *mut uiWindowsControl;

    pub fn uiWindowsEnsureCreateControlHWND(dwExStyle: DWORD,
                                            lpClassName: LPCWSTR,
                                            lpWindowName: LPCWSTR,
                                            dwStyle: DWORD,
                                            hInstance: HINSTANCE,
                                            lpParam: LPVOID,
                                            useStandardControlFont: BOOL)
                                            -> HWND;

    pub fn uiWindowsEnsureDestroyWindow(hwnd: HWND);

    pub fn uiWindowsEnsureSetParentHWND(hwnd: HWND, parent: HWND);

    pub fn uiWindowsEnsureAssignControlIDZOrder(hwnd: HWND,
                                                controlID: *mut LONG_PTR,
                                                insertAfter: *mut HWND);

    pub fn uiWindowsEnsureGetClientRect(hwnd: HWND, r: *mut RECT);
    pub fn uiWindowsEnsureGetWindowRect(hwnd: HWND, r: *mut RECT);

    pub fn uiWindowsWindowText(hwnd: HWND) -> *mut c_char;
    pub fn uiWindowsSetWindowText(hwnd: HWND, text: *const c_char);

    pub fn uiWindowsWindowTextWidth(hwnd: HWND) -> intmax_t;

    pub fn uiWindowsEnsureMoveWindowDuringResize(hwnd: HWND,
                                                 x: intmax_t,
                                                 y: intmax_t,
                                                 width: intmax_t,
                                                 height: intmax_t);

    pub fn uiWindowsRegisterWM_COMMANDHandler(hwnd: HWND,
                                              handler: extern "C" fn(*mut uiControl,
                                                                     HWND,
                                                                     WORD,
                                                                     *mut LRESULT)
                                                                     -> BOOL,
                                              c: *mut uiControl);
    pub fn uiWindowsUnregisterWM_COMMANDHandler(hwnd: HWND);

    pub fn uiWindowsRegisterWM_NOTIFYHandler(hwnd: HWND,
                                              handler: extern "C" fn(*mut uiControl,
                                                                     HWND,
                                                                     *mut NMHDR,
                                                                     *mut LRESULT)
                                                                     -> BOOL,
                                              c: *mut uiControl);
    pub fn uiWindowsUnregisterWM_NOTIFYHandler(hwnd: HWND);

    pub fn uiWindowsRegisterWM_HSCROLLHandler(hwnd: HWND,
                                              handler: extern "C" fn(*mut uiControl,
                                                                     HWND,
                                                                     WORD,
                                                                     *mut LRESULT)
                                                                     -> BOOL,
                                              c: *mut uiControl);
    pub fn uiWindowsUnregisterWM_HSCROLLHandler(hwnd: HWND);

    pub fn uiWindowsRegisterReceiveWM_WININICHANGE(hwnd: HWND);
    pub fn uiWindowsUnregisterReceiveWM_WININICHANGE(hwnd: HWND);
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct uiWindowsSizing {
    pub BaseX: c_int,
    pub BaseY: c_int,
    pub InternalLeading: LONG,
}

#[link(name = "ui")]
extern {
    pub fn uiWindowsGetSizing(hwnd: HWND, sizing: *mut uiWindowsSizing);
    pub fn uiWindowsSizingDlgUnitsToPixels(sizing: *mut uiWindowsSizing,
                                           x: *mut c_int,
                                           y: *mut c_int);
    pub fn uiWindowsSizingStandardPadding(sizing: *mut uiWindowsSizing,
                                          x: *mut c_int,
                                          y: *mut c_int);

    pub fn uiWindowsMakeContainer(c: *mut uiWindowsControl,
                                  onResize: extern "C" fn(*mut uiWindowsControl) -> HWND)
                                  -> HWND;

    pub fn uiWindowsControlTooSmall(c: *mut uiWindowsControl) -> BOOL;
    pub fn uiWindowsControlContinueMinimumSizeChanged(c: *mut uiWindowsControl);

    pub fn uiWindowsControlAssignSoleControlIDZOrder(control: *mut uiWindowsControl);

    pub fn uiWindowsShouldStopSyncEnableState(c: *mut uiWindowsControl, enabled: c_int) -> BOOL;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NMHDR {
    pub hwndFrom: HWND,
    pub idFrom: UINT_PTR,
    pub code: UINT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

pub type BOOL = c_int;

pub type DWORD = c_ulong;

pub type HANDLE = PVOID;

pub type HINSTANCE = HANDLE;

pub type HWND = HANDLE;

pub type LONG = c_long;

#[cfg(target_pointer_width = "64")]
pub type LONG_PTR = i64;
#[cfg(not(target_pointer_width = "64"))]
pub type LONG_PTR = c_long;

pub type LPCWSTR = *const WCHAR;

pub type LPVOID = *mut c_void;

pub type LRESULT = LONG_PTR;

pub type PVOID = *mut c_void;

pub type UINT = c_uint;

#[cfg(target_pointer_width = "64")]
pub type UINT_PTR = u64;
#[cfg(not(target_pointer_width = "64"))]
pub type UINT_PTR = c_uint;

pub type WCHAR = wchar_t;

pub type WORD = c_ushort;

